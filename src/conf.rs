use error::{Error, Result};
use files;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conf {
    pub force: Option<bool>,
    pub full: Option<bool>,
    pub in_dir: Option<String>,
    pub out_dir: Option<String>,
    pub port: Option<u16>,
    pub title: Option<String>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            full: None,
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
        toml::from_slice(files::fread(path)?.as_slice())
            .map_err(|e| Error::new(format!("error parsing configuration: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::{env, fs};
    use ::std::fs::File;
    use ::std::io::Write;

    #[test]
    fn test_conf_defeault_value() {
        let c = Conf::default();
        assert!(c.full == None);
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
        f.write(b"force = true\n\
                  in_dir = \".\"\n\
                  port = 9999\n\
                  title = \"title\"").unwrap();

        let c = Conf::from_file(&p).unwrap();
        assert!(c.full == None);
        assert!(c.force == Some(true));
        assert!(c.in_dir == Some(".".to_string()));
        assert!(c.out_dir == None);
        assert!(c.port == Some(9999));
        assert!(c.title == Some("title".to_string()));

        fs::remove_file(p).unwrap();
    }
}
