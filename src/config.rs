use error::Result;
use std::fmt::Debug;
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
    pub fn from_file<P>(path: P) -> Result<Self>
            where P: AsRef<Path> + Debug {
        let mut reader = File::open(&path)
            .map_err(|e| format!("open {:?} failed: {}", path, e))?;
        let mut conf = vec![];
        reader.read_to_end(&mut conf)
              .map_err(|e| format!("reading config file failed: {}", e))?;
        let config = toml::from_slice(conf.as_slice())
            .map_err(|e| format!("parse config file failed: {}", e))?;

        Ok(config)
    }
}
