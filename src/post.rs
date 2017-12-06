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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        // call format to make sure the leading zero exists
        ctx.add("year", &self.ts.year());
        ctx.add("month", &self.ts.format("%m").to_string());
        ctx.add("day", &self.ts.format("%d").to_string());
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
    use ::std::fs::{OpenOptions, File, remove_file};
    use ::std::io::Write;
    use ::std::path::PathBuf;

    fn assert_create(kind: PostKind) {
        let mut c = Conf::default();
        c.force = Some(true);
        let just_now = Local::now();

        let path = env::temp_dir().join("x.md");
        create_post(&path, c.clone(), kind.clone()).unwrap();

        fs::OpenOptions::new().append(true)
            .open(&path).unwrap()
            .write(b"XXX").unwrap();

        let post = Post::from_file(&path).unwrap().unwrap();

        assert!(just_now < post.ts && post.ts < Local::now());
        assert!(&post.link == "x");
        assert!(post.kind == kind);
        assert!(post.content == markdown::markdown_to_html("XXX").unwrap().into_bytes());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_create_post() {
        assert_create(PostKind::Article);
        assert_create(PostKind::Page);
    }

    fn temp_src() -> (PathBuf, File) {
        let path = env::temp_dir().join("y.md");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path).unwrap();
        (path, file)
    }

    #[test]
    fn test_post_with_bad_meta() {
        // no meta at all
        let (path, _) = temp_src();
        let post = Post::from_file(&path);
        assert!(post.is_ok());
        assert!(post.unwrap().is_none());
        remove_file(path).unwrap();

        // only a meta ending mark
        let (path, mut file) = temp_src();
        file.write(POST_META_MARK.as_bytes()).unwrap();
        let post = Post::from_file(&path);
        assert!(post.is_err());
        remove_file(path).unwrap();

        // zero-length URL
        let (path, mut file) = temp_src();
        file.write(b"title = \"xxx\"\n\
                     link = \"yyy\"\n\
                     url = \"\"\n\
                     ts = \"2017-12-04T20:23:37.463860-05:00\"\n\
                     kind = \"Page\"\n")
            .unwrap();
        file.write(POST_META_MARK.as_bytes()).unwrap();

        let post = Post::from_file(&path);
        assert!(post.is_err());

        remove_file(path).unwrap();
    }
}
