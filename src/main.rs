const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

static FORMAT: &'static str = "{request-time}: {method} {uri} {status} {response-time}";

#[macro_use]
extern crate clap;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};

extern crate toml;
extern crate rustc_serialize;
extern crate regex;

extern crate iron;
use iron::prelude::*;
use iron::Handler;
use iron::method::Method;
use std::str::FromStr;

extern crate router;
use router::Router;

extern crate staticfile;
use staticfile::Static;

extern crate mount;
use mount::Mount;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{SimpleLogger, LogLevelFilter};

extern crate logger;
use logger::Logger;
use logger::Format;

extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use notify::DebouncedEvent;
use notify::DebouncedEvent::{Create, Write};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use std::thread;
use std::path::PathBuf;

mod config;
use config::{Config, ServerConfig, RequestConfig, ResponseConfig};
use std::collections::HashMap;
mod error;
use error::HaxoniteError;
mod handlers;
use handlers::*;
mod utils;

use std::path::Path;

fn main() {
    let _ = SimpleLogger::init(LogLevelFilter::Info, simplelog::Config::default());

    match run() {
        Ok(_) => {}
        Err(err) => {
            error!("{}!", err);
            std::process::exit(-1);
        }
    }
}

fn run() -> Result<(), HaxoniteError> {
    let matches = define_command_line_options().get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        let generate_full_project = matches.is_present("full");
        if let Some(project_name) = matches.value_of("project_name") {
            try!(utils::create_new_project(project_name, generate_full_project));
        }
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("serve") {
        try!(serve(matches, None));
    }
    if let Some(matches) = matches.subcommand_matches("watch") {
        try!(watch(matches));
    }
    Ok(())
}

fn serve(matches: &ArgMatches, rx_watcher: Option<Receiver<DebouncedEvent>>) -> Result<(), HaxoniteError> {
    let config_file = matches.value_of("config_file").unwrap_or("config.toml");
    'serve: loop {
        let config_content = try!(config::read_config(config_file));
        let config: Config = toml::decode_str(config_content.as_ref()).unwrap(); // TODO: remove unwrap to handle malformed config
        debug!("Using config: {:?}!", config);

        let server_config = config.server.unwrap_or_else(ServerConfig::default);
        let host = match value_t!(matches, "host", String) { // TODO: rewrite these matches
            Ok(host) => host,
            Err(_) => server_config.host.unwrap_or_else(config::default_host),
        };
        let port_number = match value_t!(matches, "port_number", u16) {
            Ok(port_number) => port_number,
            Err(_) => server_config.port.unwrap_or_else(config::default_port),
        };

        // Initialize Iron's router and pass it to config processing function to define routes
        let mut router = Router::new();
        let mut mount = Mount::new();
        match config.requests {
            Some(requests) => {
                try!(process_config_requests(&requests, &mut mount, &mut router));
            }
            None => return Err(HaxoniteError::NoRequestDefined),
        }
        mount.mount("/", router);

        // Insert logger middlewares before and after router
        let mut chain = Chain::new(mount);
        let format = Format::new(FORMAT);
        let (logger_before, logger_after) = Logger::new(Some(format.unwrap()));
        chain.link_before(logger_before);
        chain.link_after(logger_after);

        // Initialize Iron to process requests
        let mut _iron = try!(Iron::new(chain).http((host.as_ref(), port_number)));
        info!("Haxonite running on port: {}!", port_number);

        if let Some(ref rx_watcher) = rx_watcher {
            'watch: loop {
                match rx_watcher.recv() {
                    Ok(Create(path)) | Ok(Write(path)) => {
                        match PathBuf::from(&path).ends_with(config_file) {
                            true => {
                                info!("Reloading...");
                                _iron.close().expect("Iron cannot be closed");
                                thread::sleep(Duration::from_secs(1));
                                break 'watch
                            },
                            false => continue 'watch
                        }
                    },
                    Ok(_) => continue 'watch,
                    Err(e) => error!("watch error: {:?}", e),
                }
            }
        }
    }
}

fn watch(matches: &ArgMatches) -> Result<(), HaxoniteError> {
    let config_file = PathBuf::from(matches.value_of("config_file").unwrap_or("config.toml"));

    let (tx_watcher, rx_watcher) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx_watcher, Duration::from_secs(1)));
    let dir = config_file.parent().unwrap();
    try!(watcher.watch(dir, RecursiveMode::NonRecursive)); // NonRecursie as we only watch config file

    serve(matches, Some(rx_watcher))
}

fn process_config_requests(requests: &HashMap<String, RequestConfig>,
                           mut mount: &mut Mount,
                           mut router: &mut Router)
                           -> Result<(), HaxoniteError> {
    info!("Processing config!");
    for (request_name, request_config) in requests {
        info!("Processing config for {}: {:?}!",
              request_name,
              request_config);
        try!(define_route(request_name.clone(),
                          request_config.clone(),
                          &mut mount,
                          &mut router));
    }
    Ok(())
}

fn define_route(request_name: String, request_config: RequestConfig, mount: &mut Mount, router: &mut Router) -> Result<(), HaxoniteError> {
    let method = request_config.method.unwrap_or_else(config::default_method);
    let path = match request_config.path {
        Some(path) => path,
        None => return Err(HaxoniteError::NoPathDefined(request_name.clone())),
    };

    let type_ = request_config.type_.unwrap_or_else(config::default_type);
    let response_configs = match request_config.responses {
        Some(response_configs) => response_configs,
        None => return Err(HaxoniteError::NoResponseDefined(request_name.clone())),
    };
    let handler: Box<dyn Handler> = match type_.as_ref() {
        "single" => {
            let response_config = try!(response_configs.first()
                .ok_or(HaxoniteError::NoResponseDefined(request_name.clone())));
            Box::new(try!(get_single_response_handler(request_name.clone(), response_config.clone())))
        }
        "random" => {
            let mut handler = RandomResponse::new();
            for response_config in &response_configs {
                let weight = response_config.weight.unwrap_or_else(config::default_weight);
                handler.add_handler(weight,
                                    try!(get_single_response_handler(request_name.clone(), response_config.clone())));
            }
            Box::new(handler)
        }
        "roundrobin" => {
            let mut handler = IteratingResponse::new("roundrobin");
            for response_config in &response_configs {
                handler.add_handler(try!(get_single_response_handler(request_name.clone(), response_config.clone())));
            }
            Box::new(handler)
        }
        "chain" => {
            let mut handler = IteratingResponse::new("chain");
            for response_config in &response_configs {
                handler.add_handler(try!(get_single_response_handler(request_name.clone(), response_config.clone())));
            }
            Box::new(handler)
        }
        "static" => {
            let response_config = try!(response_configs.first()
                .ok_or(HaxoniteError::NoResponseDefined(request_name.clone())));
            let response = match response_config.response {
                Some(ref response) => response,
                None => return Err(HaxoniteError::NoResponseDefined(request_name.clone())),
            };
            let handler = Static::new(Path::new(&response));
            Box::new(handler)
        }
        _ => return Err(HaxoniteError::InvalidType(request_name.clone())),
    };

    match Method::from_str(method.to_ascii_uppercase().as_ref()) {
        Ok(Method::Extension(_)) |
        Err(_) => return Err(HaxoniteError::InvalidHTTPMethod(request_name.clone())),
        Ok(a) => {
            match type_.as_ref() {
                "static" => {
                    info!("Mounting static for: {} using {} type of handler.",
                          path,
                          type_);
                    mount.mount(path.as_ref(), handler);
                }
                _ => {
                    info!("Defining route for: {} using {} type of handler.",
                          path,
                          type_);
                    router.route(a, path.clone(), handler, request_name);
                }
            }
        }
    };
    Ok(())
}

fn get_single_response_handler(request_name: String, response_config: ResponseConfig) -> Result<SingleResponse, HaxoniteError> {
    let headers = process_headers(response_config.headers);
    let status = match response_config.status {
        Some(status) if status >= 100 && status <= 599 => status, // switch to Range's contains() when it will be stable
        Some(status) => return Err(HaxoniteError::InvalidHTTPStatus(request_name.clone(), status)),
        None => config::default_status(),
    };
    let response = match response_config.response {
        Some(response) => response,
        None => return Err(HaxoniteError::NoResponseDefined(request_name.clone())),
    };
    if !Path::new(&response).exists() {
        return Err(HaxoniteError::ResponseDoesNotExist(request_name.clone()));
    }
    Ok(SingleResponse::new(status, headers, response, response_config.delay))
}

fn process_headers(headers: Option<Vec<String>>) -> Vec<Vec<String>> {
    // TODO: This method should be optimized to not compile regex on every run,
    // not match on header when the default is used (it's valid)
    use regex::Regex;
    let re = Regex::new(r"([\w-]+):\s+([\w/]+)").unwrap();

    let headers = headers.unwrap_or_else(config::default_headers);
    headers.iter()
        .filter(|header| {
            if re.is_match(header) {
                true
            } else {
                warn!("Header {} is invalid (does not match \": \"). Skipping.",
                      header);
                false
            }
        })
        .map(|header| {
            header.split(": ")
                .map(// TODO: This is quite stupid and could be done with &str,
                     // but I decided to use String to not fight borrow checker for now...
                     |hdr| hdr.to_string())
                .collect()
        })
        .collect()
}

fn define_command_line_options<'a, 'b>() -> App<'a, 'b> {
    App::new("Haxonite")
        .setting(AppSettings::SubcommandRequired)
        .version(VERSION)
        .author(AUTHORS)
        .about("Easy API mocking")
        .subcommand(SubCommand::with_name("serve")
            .about("Starts Haxonite server")
            .version(VERSION)
            .author(AUTHORS)
            .arg(Arg::with_name("host")
                 .short("h")
                 .long("host")
                 .value_name("HOST")
                 .help("Run Haxonite on the host. Default: localhost.")
                 .takes_value(true))
            .arg(Arg::with_name("port_number")
                 .short("p")
                 .long("port")
                 .value_name("PORT")
                 .help("Run Haxonite on the specified port. Default: 4000.")
                 .takes_value(true))
            .arg(Arg::with_name("config_file")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file. Default: config.toml.")
                 .takes_value(true)))
        .subcommand(SubCommand::with_name("watch")
            .about("Starts Haxonite server with dynamic reload on change")
            .version(VERSION)
            .author(AUTHORS)
            .arg(Arg::with_name("host")
                 .short("h")
                 .long("host")
                 .value_name("HOST")
                 .help("Run Haxonite on the host. Default: localhost.")
                 .takes_value(true))
            .arg(Arg::with_name("port_number")
                 .short("p")
                 .long("port")
                 .value_name("PORT")
                 .help("Run Haxonite on the specified port. Default: 4000.")
                 .takes_value(true))
            .arg(Arg::with_name("config_file")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file. Default: config.toml.")
                 .takes_value(true)))
        .subcommand(SubCommand::with_name("new")
            .about("Create new Haxonite project")
            .version(VERSION)
            .author(AUTHORS)
            .arg(Arg::with_name("full")
                .help("Use to create a project with full config.toml file")
                .short("f")
                .long("full"))
            .arg(Arg::with_name("project_name")
                .help("Name of the project (will be used as a directory name)")
                .index(1)
                .required(true)))
}
