use std::{error, fmt, io, result, string};
use std::ops::Deref;
use tera;
use toml;

#[derive(Debug)]
pub struct Error {
    msg: String,
    origin: Option<Box<error::Error>>,
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new(msg: &str, origin: Option<Box<error::Error>>) -> Self {
        Error { msg: msg.to_string(), origin }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.origin {
            Some(ref ptr) => ptr.deref().description(),
            None => ""
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.origin {
            Some(ref ptr) => Some(ptr.deref()),
            None => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.origin {
            Some(ref ptr) => write!(f, "{}: {}", self.msg, ptr.deref()),
            None => write!(f, "{}", self.msg)
        }
    }
}

// XXX macro is your friend
impl From<string::String> for Error {
    fn from(msg: string::String) -> Self {
        Error { msg, origin: None }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { msg: e.to_string(), origin: Some(Box::new(e)) }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error { msg: e.to_string(), origin: Some(Box::new(e)) }
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error { msg: e.to_string(), origin: Some(Box::new(e)) }
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Error { msg: e.to_string(), origin: Some(Box::new(e)) }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error { msg: e.to_string(), origin: Some(Box::new(e)) }
    }
}
