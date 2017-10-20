extern crate chrono;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate time;
extern crate toml;

pub mod config;
pub mod error;
pub mod gen;
pub mod post;

pub const CONFIG_FILE: &str = ".izzetconfig";
// XXX this should be made configurable
pub const NOJEKYLL_FILE: &str = ".nojekyll";

pub const FILES_DIR: &str = "files";
pub const SRC_DIR: &str = "src";
pub const TEMPLATES_DIR: &str = "templates";

pub const INDEX_FILE: &str = "index.html";
pub const POST_FILE: &str = "post.html";
pub const ARCHIVE_FILE: &str = "archive.html";

pub const INDEX_HTML: &[u8] = b"\
<!DOCTYPE html>
<html>
<head>
  <meta charset=\"utf-8\">
</head>
<body>
  <h1><a href=\"/\">{{ config.title }}</a></h1>
  <div>
    <div>
      {{ post.content }}
    </div>
  </div>
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
    {% for post in posts %}
      <li><a href=\"/\">{{ post.title }}</a></li>
    {% endfor %}
    </ul>
  </div>
</body>
</html>
";
