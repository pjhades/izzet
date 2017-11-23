use error::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub force: Option<bool>,
    pub in_dir: Option<String>,
    pub out_dir: Option<String>,
    pub port: Option<u16>,
    pub title: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            force: None,
            in_dir: None,
            out_dir: None,
            port: None,
            title: "Default Title".to_string(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::{env, fs};
    use ::std::io::Write;

    #[test]
    fn test_read_config() {
        let p = env::temp_dir().join("config.test");
        let mut f = File::create(&p).unwrap();
        f.write(b"force = true\n\
                  in_dir = \".\"\n\
                  port = 9999\n\
                  title = \"title\"").unwrap();

        let c = Config::from_file(&p).unwrap();
        assert!(c.force == Some(true));
        assert!(c.in_dir == Some(".".to_string()));
        assert!(c.out_dir == None);
        assert!(c.port == Some(9999));
        assert!(c.title == "title".to_string());

        fs::remove_file(p).unwrap();
    }
}
