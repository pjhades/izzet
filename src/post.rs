use chrono::{DateTime, Local};
use conf::Conf;
use error::{Error, Result};
use files;
use markdown;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
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
    pub content: Vec<u8>,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            meta: PostMeta::default(),
            content: vec![],
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let mut reader = File::open(&path).map(|f| BufReader::new(f))
            .map_err(|e| format!("error opening {:?}: {}", path.as_ref(), e))?;

        let mut meta = "".to_string();
        let mut line = "".to_string();
        loop {
            match reader.read_line(&mut line) {
                Ok(n) if n > 0 && &line != POST_META_MARK => {
                    meta += &line;
                    line.clear();
                },
                Ok(n) if n > 0 => break,
                _ => return Ok(None),
            }
        }

        let meta: PostMeta = toml::from_str(&meta)
            .map_err(|e| format!("error parsing metadata of {:?}: {}", path.as_ref(), e))?;

        let mut content = vec![];
        reader.read_to_end(&mut content)
              .map_err(|e| format!("error reading content from {:?}: {}", path.as_ref(), e))?;

        let content = match path.as_ref().extension().and_then(|s| s.to_str()) {
            Some("md") | Some("markdown") =>
                markdown::markdown_to_html(str::from_utf8(&content)?)?.into_bytes(),
            _ =>
                content,
        };

        Ok(Some(Post { meta, content }))
    }
}

pub fn create_post<P: AsRef<Path>>(path: P, conf: Conf, kind: PostKind) -> Result<()> {
    let link = match path.as_ref().file_stem().and_then(|s| s.to_str()) {
        None => return Err(Error::new("need specify link of post".to_string())),
        Some(stem) => stem.to_string(),
    };

    let mut post = Post::new();
    post.meta.kind = kind;
    post.meta.link = link.to_string();

    let content = toml::to_string(&post.meta)? + POST_META_MARK;
    files::fwrite(&path, content.as_bytes(), conf.force.unwrap_or(false))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::{env, fs};
    use ::std::io::Write;

    #[test]
    fn test_post_default_value() {
        let just_now = Local::now();
        let meta = PostMeta::default();
        assert!(&meta.title == "Default Title");
        assert!(&meta.link == "default-link");
        assert!(just_now < meta.ts && meta.ts < Local::now());
        assert!(meta.kind == PostKind::Article);
    }

    #[test]
    fn test_create_post() {
        let mut c = Conf::default();
        c.force = Some(true);

        let p = env::temp_dir().join("article.md");
        let just_now = Local::now();
        create_post(&p, c, PostKind::Article).unwrap();
        fs::OpenOptions::new().append(true).open(&p).unwrap().write(b"XXX").unwrap();

        let post = Post::from_file(&p).unwrap().unwrap();
        assert!(just_now < post.ts && post.ts < Local::now());
        assert!(&post.title == "Default Title");
        assert!(&post.link == "article");
        assert!(post.kind == PostKind::Article);
        assert!(post.content == markdown::markdown_to_html("XXX").unwrap().into_bytes());

        fs::remove_file(p).unwrap();
    }
}
