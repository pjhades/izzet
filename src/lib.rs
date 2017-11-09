extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate toml;

use std::fs::OpenOptions;

pub mod config;
pub mod error;
pub mod gen;
pub mod post;

pub const CONFIG_FILE:   &str = ".izzetconfig";
// XXX this should be made configurable
pub const NOJEKYLL_FILE: &str = ".nojekyll";
pub const INDEX_FILE:    &str = "index.html";
pub const POST_FILE:     &str = "post.html";
pub const ARCHIVE_FILE:  &str = "archive.html";
pub const FILES_DIR:     &str = "files";
pub const ARTICLES_DIR:   &str = "articles";
pub const PAGES_DIR:     &str = "pages";
pub const TEMPLATES_DIR: &str = "templates";

pub const SITE_DIRS:  &[&str] = &[
    FILES_DIR,
    ARTICLES_DIR,
    PAGES_DIR,
    TEMPLATES_DIR
];

pub const SITE_FILES: &[&str] = &[
    NOJEKYLL_FILE,
    CONFIG_FILE
];

pub const SITE_TEMPLATES: &[(&str, &[u8])] = &[
    (POST_FILE,    POST_HTML),
    (INDEX_FILE,   INDEX_HTML),
    (ARCHIVE_FILE, ARCHIVE_HTML),
];

// HTML for the default template
pub const POST_HTML: &[u8] = b"\
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
</head>
<body>
  <h1><a href=\"/\">{{ post.meta.title }}</a></h1>
  <div>
    <div>
      {{ post.content }}
    </div>
  </div>
</body>
</html>
";

pub const INDEX_HTML: &[u8] = b"\
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
</head>
<body>
  {% if latest_article %}
  <h1><a href=\"/\">{{ latest_article.meta.title }}</a></h1>
  <div>
    <div>
      {{ latest_article.content }}
    </div>
  </div>
  {% endif %}
</body>
</html>
";

pub const ARCHIVE_HTML: &[u8] = b"\
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
</head>
<body>
  <h1><a href=\"/\">{{ config.title }}</a></h1>
  <div>
    <ul>
    {% for article in articles %}
      <li><a href=\"/\">{{ article.meta.title }}</a></li>
    {% endfor %}
    </ul>
  </div>
</body>
</html>
";

pub fn get_opener(force: bool) -> OpenOptions {
    let mut opener = OpenOptions::new();
    opener.write(true);
    if force {
        opener.create(true).truncate(true);
    }
    else {
        opener.create_new(true);
    }
    opener
}
