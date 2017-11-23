use std::{fmt, io, result, string};
use tera;
use toml;

#[derive(Debug)]
pub struct Error {
    msg: String
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new(msg: String) -> Self {
        Error { msg }
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

impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<Box<::std::error::Error + Send + Sync + 'static>> for Error {
    fn from(e: Box<::std::error::Error + Send + Sync + 'static>) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error { msg: "error from tiny_http header parsing".to_string() }
    }
}
