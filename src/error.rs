use std::{error, fmt, io, result, string};
use tera;
use toml;

#[derive(Debug)]
pub struct Error {
    msg: String
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error { msg }
    }
}

pub type Result<T> = result::Result<T, Error>;

trait ResultContext<T, E> {
    fn context(self, s: String) -> Result<T>;
}

impl<T, E> ResultContext<T, E> for result::Result<T, E>
        where E: error::Error
{
    fn context(self, s: String) -> Result<T> {
        self.map_err(|e| Error::new(format!("{}: {}", s, e.description())))
    }
}

impl<E: error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        Error::new(e.description().to_string())
    }
}
