use chrono::{DateTime, Datelike, Local};
use conf::Conf;
use error::{Error, Result, ResultContext};
use files;
use markdown;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::ops::Deref;
use std::path::Path;
use std::str;
use std::string::String;
use tera::{Tera, Context};
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
    pub url: String,
    pub ts: DateTime<Local>,
    pub kind: PostKind,
}

const DEFAULT_TITLE: &str = "Default Title";
const DEFAULT_LINK: &str = "default-link";
const DEFAULT_ARTICLE_URL: &str = "/{{ year }}/{{ month }}/{{ day }}/{{ link }}.html";
const DEFAULT_PAGE_URL: &str = "/{{ link }}.html";

impl Default for PostMeta {
    fn default() -> Self {
        PostMeta {
            title: DEFAULT_TITLE.to_string(),
            link: DEFAULT_LINK.to_string(),
            url: DEFAULT_ARTICLE_URL.to_string(),
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
            .context(format!("error opening {:?}", path.as_ref()))?;

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
            .context(format!("error parsing metadata of {:?}", path.as_ref()))?;
        // XXX maybe add more metadata sanity check here
        // as later we'll lose the corresponding file path
        // when generating it
        if meta.url.len() == 0 {
            return Err(Error::new(format!("output URL of post {:?} is 0", path.as_ref())));
        }

        let mut content = vec![];
        reader.read_to_end(&mut content)
              .context(format!("error reading content from {:?}", path.as_ref()))?;

        let content = match path.as_ref().extension().and_then(|s| s.to_str()) {
            Some("md") | Some("markdown") =>
                markdown::markdown_to_html(str::from_utf8(&content)?)?.into_bytes(),
            _ =>
                content,
        };

        Ok(Some(Post { meta, content }))
    }

    pub fn url(&self) -> Result<String> {
        let mut ctx = Context::new();
        ctx.add("year", &self.ts.year());
        ctx.add("month", &self.ts.month());
        ctx.add("day", &self.ts.day());
        ctx.add("link", &self.link);
        Tera::one_off(&self.url, &ctx, false)
            .map_err(|e| Error::from(e))
    }
}

pub fn create_post<P: AsRef<Path>>(path: P, conf: Conf, kind: PostKind) -> Result<()> {
    let link = match path.as_ref().file_stem().and_then(|s| s.to_str()) {
        None => return Err(Error::new("need specify link of post".to_string())),
        Some(stem) => stem.to_string(),
    };

    let mut post = Post::new();
    post.meta.url = match &kind {
        &PostKind::Article => DEFAULT_ARTICLE_URL.to_string(),
        &PostKind::Page => DEFAULT_PAGE_URL.to_string(),
    };
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
        assert!(&meta.url == DEFAULT_ARTICLE_URL);
        assert!(just_now < meta.ts && meta.ts < Local::now());
        assert!(meta.kind == PostKind::Article);
    }

    #[test]
    fn test_create_post() {
        let mut c = Conf::default();
        c.force = Some(true);
        let just_now = Local::now();

        let article_path = env::temp_dir().join("temp-article.md");
        create_post(&article_path, c.clone(), PostKind::Article).unwrap();
        fs::OpenOptions::new().append(true).open(&article_path).unwrap().write(b"XXX").unwrap();

        let page_path = env::temp_dir().join("temp-page.md");
        create_post(&page_path, c, PostKind::Page).unwrap();
        fs::OpenOptions::new().append(true).open(&page_path).unwrap().write(b"YYY").unwrap();

        let article = Post::from_file(&article_path).unwrap().unwrap();
        assert!(just_now < article.ts && article.ts < Local::now());
        assert!(&article.link == "temp-article");
        assert!(article.kind == PostKind::Article);
        assert!(article.content == markdown::markdown_to_html("XXX").unwrap().into_bytes());
        fs::remove_file(article_path).unwrap();

        let page = Post::from_file(&page_path).unwrap().unwrap();
        assert!(just_now < page.ts && page.ts < Local::now());
        assert!(&page.link == "temp-page");
        assert!(page.kind == PostKind::Page);
        assert!(page.content == markdown::markdown_to_html("YYY").unwrap().into_bytes());
        fs::remove_file(page_path).unwrap();
    }

    #[test]
    fn test_post_with_bad_meta() {
        // no meta at all
        let path = env::temp_dir().join("temp-post.md");
        fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap()
            .write(b"XXX").unwrap();
        let post = Post::from_file(&path);
        assert!(post.is_ok());
        assert!(post.unwrap().is_none());
        fs::remove_file(path).unwrap();

        // only a meta ending mark
        let path = env::temp_dir().join("temp-post.md");
        fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap()
            .write(POST_META_MARK.as_bytes()).unwrap();
        let post = Post::from_file(&path);
        assert!(post.is_err());
        fs::remove_file(path).unwrap();

        // zero-length URL
        let path = env::temp_dir().join("temp-post.md");
        let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
        file.write(b"\
title = \"xxx\"
link = \"yyy\"
url = \"\"
ts = \"2017-12-04T20:23:37.463860-05:00\"
kind = \"Page\"
")
            .unwrap();
        file.write(POST_META_MARK.as_bytes()).unwrap();
        let post = Post::from_file(&path);
        assert!(post.is_err());
        fs::remove_file(path).unwrap();
    }
}
