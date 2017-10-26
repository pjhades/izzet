use chrono::{DateTime, Local};
use error::Result;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::Path;
use std::string::String;
use toml;

pub const METADATA_DELIM_LINE: &str = "---\n";

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub link: String,
    pub timestamp: DateTime<Local>,
}

#[derive(Serialize)]
pub struct Post {
    pub meta: Meta,
    pub content: String,
}

impl Post {
    pub fn from_path(path: &Path) -> Result<Self> {
        let reader = File::open(path)
                     .map_err(|e| format!("open {:?} failed: {}", path, e))?;
        let mut reader = BufReader::new(reader);

        // parse metadata
        let mut meta = "".to_string();
        let mut line = "".to_string();;
        while line != METADATA_DELIM_LINE {
            meta += &line;
            line.clear();
            reader.read_line(&mut line)
                  .map_err(|e| format!("read metadata from {:?} failed: {}", path, e))?;
        }

        let meta: Meta = toml::from_str(&meta)
                         .map_err(|e| format!("parse metadata of {:?} failed: {}", path, e))?;

        // parse content
        let mut content = vec![];
        reader.read_to_end(&mut content)
              .map_err(|e| format!("read content from {:?} failed: {}", path, e))?;

        Ok(Post {
            meta,
            content: String::from_utf8(content)?,
        })
    }
}
