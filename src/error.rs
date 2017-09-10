use std::{fmt, io, result};

pub struct Error {
    msg: String
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error { msg: msg.to_string() }
    }

    pub fn from_string(msg: String) -> Self {
        Error { msg }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

pub type Result<T> = result::Result<T, Error>;
