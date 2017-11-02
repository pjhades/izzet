use error::{Error, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

// XXX make some of them optional
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        let mut reader = File::open(path)
                         .map_err(|e| Error::new(format!("open {:?} failed: {}", path, e),
                                                 Some(Box::new(e))))?;
        let mut conf = vec![];
        reader.read_to_end(&mut conf)
              .map_err(|e| Error::new(format!("reading config file failed: {}", e),
                                      Some(Box::new(e))))?;

        let config = toml::from_slice(conf.as_slice())
                     .map_err(|e| Error::new(format!("parse config file failed: {}", e),
                                             Some(Box::new(e))))?;

        Ok(config)
    }
}
