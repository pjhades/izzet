use chrono::{DateTime, Local};
use config::Config;
use error::Result;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::PathBuf;
use std::string::String;
use std::io::Write;
use toml;

const POST_META_END: &str = "%%%\n";

#[derive(Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub link: String,
    pub ts: DateTime<Local>,
    pub is_article: bool,
}

impl Default for PostMeta {
    fn default() -> Self {
        PostMeta {
            title: "Default Title".to_string(),
            link: "default-link".to_string(),
            ts: Local::now(),
            is_article: true,
        }
    }
}

#[derive(Serialize)]
pub struct Post {
    pub meta: PostMeta,
    pub path: PathBuf,
    pub content: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            meta: Default::default(),
            path: Default::default(),
            content: String::new(),
        }
    }
}

impl Post {
    pub fn new() -> Self {
        Post { ..Default::default() }
    }

    pub fn from_file(path: &PathBuf) -> Result<Self> {
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
            path: path.clone(),
            content: String::from_utf8(content)?,
        })
    }
}

pub fn create_post(link: String, config: Config, is_article: bool) -> Result<()> {
    let filename = format!("{}.md", link);
    let opener = ::get_opener(config.force.unwrap_or(false));
    let mut file = opener.open(&filename)
                         .map_err(|e| format!("fail to create {}: {}", filename, e))?;

    let mut post = Post::new();
    post.meta.link = link;
    post.meta.is_article = is_article;

    file.write(toml::to_string(&post.meta)?.as_bytes())?;
    file.write(POST_META_END.as_bytes())?;
    file.write(&post.content.as_bytes())?;

    Ok(())
}
