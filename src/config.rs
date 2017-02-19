use std::io;
use std::io::Read;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

extern crate rustc_serialize;
use rustc_serialize::{Decodable, Decoder, DecoderHelpers};

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub requests: Option<HashMap<String, RequestConfig>>,
}

#[derive(Debug, RustcDecodable, Clone)]
pub struct ServerConfig {
    pub port: Option<u16>,
    pub host: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub type_: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub responses: Option<Vec<ResponseConfig>>,
}

impl Decodable for RequestConfig {
    fn decode<D: Decoder>(d: &mut D) -> Result<RequestConfig, D::Error> {
        d.read_struct("RequestConfig", 2, |d| {
            let type_ = d.read_struct_field("type", 0, |d| d.read_str()).ok();
            let method = d.read_struct_field("method", 0, |d| d.read_str()).ok();
            let path = d.read_struct_field("path", 0, |d| d.read_str()).ok();
            let responses = d.read_struct_field("responses",
                                   0,
                                   |d| d.read_to_vec(|d| ResponseConfig::decode(d)))
                .ok();
            Ok(RequestConfig {
                type_: type_,
                method: method,
                path: path,
                responses: responses,
            })
        })
    }
}

#[derive(Debug, RustcDecodable, Clone)]
pub struct ResponseConfig {
    pub content_type: Option<String>,
    pub headers: Option<Vec<String>>,
    pub status: Option<u16>,
    pub response: Option<String>,
    pub weight: Option<u32>,
    pub delay: Option<u64>,
}

const DEFAULT_PORT: u16 = 4000;
pub fn default_port() -> u16 {
    DEFAULT_PORT
}

const DEFAULT_HOST: &'static str = "localhost";
pub fn default_host() -> String {
    DEFAULT_HOST.to_owned()
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: Some(DEFAULT_PORT),
            host: Some(DEFAULT_HOST.to_owned()),
        }
    }
}

const DEFAULT_STATUS: u16 = 200;
pub fn default_status() -> u16 {
    DEFAULT_STATUS
}

const DEFAULT_WEIGHT: u32 = 1;
pub fn default_weight() -> u32 {
    DEFAULT_WEIGHT
}

const DEFAULT_METHOD: &'static str = "GET";
pub fn default_method() -> String {
    DEFAULT_METHOD.to_owned()
}

const DEFAULT_CONTENT_TYPE: &'static str = "application/json";
pub fn default_content_type() -> String {
    DEFAULT_CONTENT_TYPE.to_owned()
}

pub fn default_headers() -> Vec<String> {
    vec!["Content-Type: application/json".to_owned()]
}

const DEFAULT_TYPE: &'static str = "single";
pub fn default_type() -> String {
    DEFAULT_TYPE.to_owned()
}

pub fn read_config(file_name: &str) -> io::Result<String> {
    let mut content = String::new();
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => return Err(io::Error::new(err.kind(), format!("{}: {}", file_name, err.description())))
    };
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(err) => return Err(io::Error::new(err.kind(), format!("{}: {}", file_name, err.description())))
    };
    Ok(content)
}
