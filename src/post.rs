use chrono::{DateTime, Local};
use config::Config;
use error::Result;
use files;
use markdown;
use std::fs::File;
use std::io::{Read, BufRead, BufReader, Write};
use std::ops::Deref;
use std::path::Path;
use std::str;
use std::string::String;
use toml;

const POST_META_MARK: &str = "%%%\n";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PostKind{
    Article,
    Page,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMeta {
    pub title: String,
    pub link: String,
    pub ts: DateTime<Local>,
    pub kind: PostKind,
}

impl Default for PostMeta {
    fn default() -> Self {
        PostMeta {
            title: "Default Title".to_string(),
            link: "default-link".to_string(),
            ts: Local::now(),
            kind: PostKind::Article,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            meta: PostMeta::default(),
            content: String::new(),
        }
    }
}

impl Deref for Post {
    type Target = PostMeta;
    fn deref(&self) -> &PostMeta {
        &self.meta
    }
}

impl Post {
    pub fn new() -> Self {
        Post::default()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let reader = File::open(&path)
            .map_err(|e| format!("open {:?} failed: {}", path.as_ref(), e))?;
        let mut reader = BufReader::new(reader);

        // parse metadata
        let mut meta = "".to_string();
        let mut line = "".to_string();
        while line != POST_META_MARK {
            meta += &line;
            line.clear();
            reader.read_line(&mut line)
                  .map_err(|e| format!("read metadata from {:?} failed: {}", path.as_ref(), e))?;
        }

        let meta: PostMeta = toml::from_str(&meta)
            .map_err(|e| format!("parse metadata of {:?} failed: {}", path.as_ref(), e))?;

        // parse content
        let mut content = vec![];
        reader.read_to_end(&mut content)
              .map_err(|e| format!("read content from {:?} failed: {}", path.as_ref(), e))?;

        Ok(Post {
            meta,
            content: markdown::markdown_to_html(str::from_utf8(&content)?)?,
        })
    }
}

pub fn create_post(link: String, config: Config, kind: PostKind) -> Result<()> {
    // XXX should support other markup languages
    let filename = format!("{}.md", link);
    let opener = files::get_opener(config.force.unwrap_or(false));
    let mut file = opener.open(&filename)
                         .map_err(|e| format!("fail to create {}: {}", filename, e))?;

    let mut post = Post::new();
    post.meta.link = link;
    post.meta.kind = kind;

    file.write(toml::to_string(&post.meta)?.as_bytes())?;
    file.write(POST_META_MARK.as_bytes())?;
    file.write(&post.content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::fs;
    use ::std::path::PathBuf;

    #[test]
    fn test_post() {
        let just_now = Local::now();
        let meta = PostMeta::default();
        assert!(&meta.title == "Default Title");
        assert!(&meta.link == "default-link");
        assert!(just_now < meta.ts && meta.ts < Local::now());
        assert!(meta.kind == PostKind::Article);
    }

    #[test]
    fn test_create_post() {
        let mut config = Config::default();
        config.force = Some(true);

        let filename = "test-link";
        let just_now = Local::now();

        create_post(filename.to_string(), config, PostKind::Article).unwrap();
        let mut path = PathBuf::from(filename);
        path.set_extension("md");

        let mut file = fs::OpenOptions::new().append(true).open(&path).unwrap();
        file.write(b"content").unwrap();

        let post = Post::from_file(&path).unwrap();
        assert!(&post.title == "Default Title");
        assert!(&post.link == "test-link");
        assert!(just_now < post.ts && post.ts < Local::now());
        assert!(post.kind == PostKind::Article);
        assert!(post.content == markdown::markdown_to_html("content").unwrap());

        fs::remove_file(path).unwrap();
    }
}
