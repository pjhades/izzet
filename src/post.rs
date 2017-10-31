use chrono::{DateTime, Local};
use error::Result;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::Path;
use std::string::String;
use toml;

pub const POST_META_END: &str = "%%%\n";

#[derive(Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub link: String,
    pub timestamp: DateTime<Local>,
}

impl Default for PostMeta {
    fn default() -> Self {
        PostMeta {
            title: "Default Title".to_string(),
            link: "default-link".to_string(),
            timestamp: Local::now(),
        }
    }
}

#[derive(Serialize)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            meta: Default::default(),
            content: "".to_string(),
        }
    }
}

impl Post {
    pub fn new() -> Self {
        Post { ..Default::default() }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let reader = File::open(path)
                     .map_err(|e| format!("open {:?} failed: {}", path, e))?;
        let mut reader = BufReader::new(reader);

        // parse metadata
        let mut meta = "".to_string();
        let mut line = "".to_string();;
        while line != POST_META_END {
            meta += &line;
            line.clear();
            reader.read_line(&mut line)
                  .map_err(|e| format!("read metadata from {:?} failed: {}", path, e))?;
        }

        let meta: PostMeta = toml::from_str(&meta)
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
