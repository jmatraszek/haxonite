extern crate iron;
extern crate notify;
use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum HaxoniteError {
    Io(io::Error),
    Server(iron::error::HttpError),
    Watch(notify::Error),
    NoRequestDefined,
    NoPathDefined(String),
    NoResponseDefined(String),
    ResponseDoesNotExist(String),
    InvalidHTTPStatus(String, u16),
    InvalidHTTPMethod(String),
    InvalidType(String),
}

impl From<io::Error> for HaxoniteError {
    fn from(err: io::Error) -> HaxoniteError {
        HaxoniteError::Io(err)
    }
}

impl From<iron::error::HttpError> for HaxoniteError {
    fn from(err: iron::error::HttpError) -> HaxoniteError {
        HaxoniteError::Server(err)
    }
}

impl From<notify::Error> for HaxoniteError {
    fn from(err: notify::Error) -> HaxoniteError {
        HaxoniteError::Watch(err)
    }
}

impl Error for HaxoniteError {
    fn description(&self) -> &str {
        match *self {
            HaxoniteError::Io(ref err) => err.description(),
            HaxoniteError::Server(ref err) => err.description(),
            HaxoniteError::Watch(ref err) => err.description(),
            HaxoniteError::NoRequestDefined => "No request defined in config.toml",
            HaxoniteError::NoPathDefined(_) => "No path defined for request in config.toml",
            HaxoniteError::NoResponseDefined(_) => "No response defined for request in config.toml",
            HaxoniteError::ResponseDoesNotExist(_) => "Response file define in config.toml does not exist",
            HaxoniteError::InvalidHTTPStatus(_, _) => "InvalidHTTPStatus",
            HaxoniteError::InvalidHTTPMethod(_) => "InvalidHTTPMethod",
            HaxoniteError::InvalidType(_) => "InvalidType",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            HaxoniteError::Io(ref err) => Some(err),
            HaxoniteError::Server(ref err) => Some(err),
            HaxoniteError::Watch(ref err) => Some(err),
            HaxoniteError::NoRequestDefined |
            HaxoniteError::NoPathDefined(_) |
            HaxoniteError::NoResponseDefined(_) |
            HaxoniteError::ResponseDoesNotExist(_) |
            HaxoniteError::InvalidHTTPStatus(_, _) |
            HaxoniteError::InvalidHTTPMethod(_) |
            HaxoniteError::InvalidType(_) => None,
        }
    }
}

impl fmt::Display for HaxoniteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HaxoniteError::Io(ref err) => write!(f, "{:?}: {}", err.kind(), err),
            HaxoniteError::Server(ref err) => write!(f, "ServerError: {}", err),
            HaxoniteError::Watch(ref err) => write!(f, "WatchError: {}", err),
            HaxoniteError::NoRequestDefined => {
                write!(f,
                       "NoRequestDefined error: No request defined in config.toml")
            }
            HaxoniteError::NoPathDefined(ref request) => {
                write!(f,
                       "NoPathDefined error: No path defined for {} request in config.toml",
                       request)
            }
            HaxoniteError::NoResponseDefined(ref request) => {
                write!(f,
                       "NoResponseDefined error: No response defined for {} request in config.toml",
                       request)
            }
            HaxoniteError::ResponseDoesNotExist(ref request) => {
                write!(f,
                       "NoResponseDefined error: Response file defined for {} request in \
                        config.toml does not exist",
                       request)
            }
            HaxoniteError::InvalidHTTPStatus(ref request, status) => {
                write!(f,
                       "InvalidHTTPStatus: Status {} defined for {} request in config.toml is not \
                        a valid HTTP status",
                       status,
                       request)
            }
            HaxoniteError::InvalidHTTPMethod(ref request) => {
                write!(f,
                       "InvalidHTTPMethod error: Invalid HTTP method defined for {} request in \
                        config.toml",
                       request)
            }
            HaxoniteError::InvalidType(ref request) => {
                write!(f,
                       "InvalidType error: Invalid type defined for {} request in config.toml",
                       request)
            }
        }
    }
}
