use error::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    pub force: Option<bool>,
    pub in_dir: Option<String>,
    pub out_dir: Option<String>,
    pub port: Option<u16>,
    pub title: Option<String>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            force: None,
            in_dir: None,
            out_dir: None,
            port: None,
            title: None,
        }
    }
}

impl Conf {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut reader = File::open(&path)
            .map_err(|e| format!("error opening configuration {:?}: {}", path.as_ref(), e))?;
        let mut conf = vec![];
        reader.read_to_end(&mut conf)
              .map_err(|e| format!("error reading configuration {:?}: {}", path.as_ref(), e))?;
        let conf = toml::from_slice(conf.as_slice())
            .map_err(|e| format!("error parsing configuration: {}", e))?;

        Ok(conf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::{env, fs};
    use ::std::io::Write;

    #[test]
    fn test_conf_defeault_value() {
        let c = Conf::default();
        assert!(c.force == None);
        assert!(c.in_dir == None);
        assert!(c.out_dir == None);
        assert!(c.port == None);
        assert!(c.title == None);
    }

    #[test]
    fn test_conf_from_file() {
        let p = env::temp_dir().join("conf.test");
        let mut f = File::create(&p).unwrap();
        f.write(b"
force = true
in_dir = \".\"
port = 9999
title = \"title\"").unwrap();

        let c = Conf::from_file(&p).unwrap();
        assert!(c.force == Some(true));
        assert!(c.in_dir == Some(".".to_string()));
        assert!(c.out_dir == None);
        assert!(c.port == Some(9999));
        assert!(c.title == Some("title".to_string()));

        fs::remove_file(p).unwrap();
    }
}
