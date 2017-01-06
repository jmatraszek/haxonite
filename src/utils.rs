// various utility functions here
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use error::HaxoniteError;

extern crate rustache;

static BARE_CONFIG_TOML: &'static str = include_str!("txt/config.toml.bare");
static FULL_CONFIG_TOML: &'static str = include_str!("txt/config.toml.full");
static LIMONITE_JSON: &'static str = include_str!("txt/haxonite.json");
static RESOURCES_JSON: &'static str = include_str!("txt/resources.json");
static RESOURCE_JSON: &'static str = include_str!("txt/resource.json");
static ERROR_404_JSON: &'static str = include_str!("txt/error.404.json");
static ERROR_500_JSON: &'static str = include_str!("txt/error.500.json");
static LIMONITE_LOGO: &'static [u8] = include_bytes!("img/haxonite.png");

pub fn response_body(response_path: String) -> String {
    let empty = "".to_string();
    let rustache_data = rustache::HashBuilder::new()
        .insert_string("haxonite_version", ::VERSION)
        .insert_string("haxonite_authors", ::AUTHORS);
    match rustache::render_file(response_path.as_ref(), rustache_data) {
        Ok(rv) => String::from_utf8(rv.into_inner()).unwrap_or(empty),
        Err(_) => empty,
    }
}

pub fn create_new_project(project_name: &str, generate_full_project: bool) -> Result<(), HaxoniteError> {
    info!("Creating new project: {}!", project_name);

    try!(fs::create_dir_all(Path::new(project_name).join("responses")));
    try!(fs::create_dir_all(Path::new(project_name).join("assets")));

    let mut config_toml = try!(File::create(Path::new(project_name).join("config.toml")));
    try!(config_toml.write_all(config_toml_content(generate_full_project)));

    let mut haxonite_json = try!(File::create(Path::new(project_name).join("responses").join("haxonite.json")));
    try!(haxonite_json.write_all(LIMONITE_JSON.as_bytes()));
    try!(try!(File::create(Path::new(project_name).join("assets").join("haxonite.png"))).write_all(LIMONITE_LOGO));

    if generate_full_project {
        try!(try!(File::create(Path::new(project_name).join("responses").join("resources.json"))).write_all(RESOURCES_JSON.as_bytes()));
        try!(try!(File::create(Path::new(project_name).join("responses").join("resource.json"))).write_all(RESOURCE_JSON.as_bytes()));
        try!(try!(File::create(Path::new(project_name).join("responses").join("error.404.json"))).write_all(ERROR_404_JSON.as_bytes()));
        try!(try!(File::create(Path::new(project_name).join("responses").join("error.500.json"))).write_all(ERROR_500_JSON.as_bytes()));
    }

    Ok(())
}

pub fn config_toml_content(generate_full_project: bool) -> &'static [u8] {
   if generate_full_project { FULL_CONFIG_TOML.as_bytes() } else { BARE_CONFIG_TOML.as_bytes() }
}
