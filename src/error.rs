use std::{fmt, io, result, string};
use tera;
use toml;

pub struct Error {
    msg: String
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new(msg: &str) -> Self {
        Error { msg: msg.to_string() }
    }
}

// XXX macro is your friend
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<string::String> for Error {
    fn from(msg: string::String) -> Self {
        Error { msg }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error { msg: e.to_string() }
    }
}
