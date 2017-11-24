extern crate chrono;
extern crate pulldown_cmark;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate tiny_http;
extern crate toml;

pub mod config;
pub mod error;
pub mod files;
pub mod markdown;
pub mod post;
pub mod server;
pub mod site;

pub const DEFAULT_PORT: u16 = 10950;

pub const CONFIG_FILE:   &str = ".izzetconfig";
// XXX this should be made configurable
pub const NOJEKYLL_FILE: &str = ".nojekyll";
pub const INDEX_FILE:    &str = "index.html";
pub const POST_FILE:     &str = "post.html";
pub const ARCHIVE_FILE:  &str = "archive.html";

pub const SRC_DIR:       &str = "src";
pub const THEME_DIR:     &str = "theme";

pub const SITE_DIRS:  &[&str] = &[
    SRC_DIR,
    THEME_DIR,
];

pub const SITE_FILES: &[&str] = &[
    NOJEKYLL_FILE,
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
  <h3>{{ post.meta.ts | date(format=\"%Y-%b-%d\") }}</h3>
  <div>
    <div>
      {{ post.content }}
    </div>
  </div>
  <div>
    <a href=\"/\">Home</a>
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
  <h1><a href=\"/\">{{ config.title }}</a></h1>
  <div>
    <ul>
      <li><a href=\"/\">Home</a></li>
      <li><a href=\"/archive.html\">Archive</a></li>
      {% for page in pages %}
        <li><a href=\"/{{ page.meta.link }}.html\">{{ page.meta.title }}</a></li>
      {% endfor %}
    </ul>
  </div>
  {% if latest_article %}
  <h2>
    <a href=\"/{{ latest_article.meta.ts | date(format=\"%Y/%m/%d\") }}/{{ latest_article.meta.link }}.html\">
    {{ latest_article.meta.title }}
    </a>
  </h2>
  <div>
    <h3>{{ latest_article.meta.ts | date(format=\"%Y-%b-%d\") }}</h3>
  </div>
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
    <li>
      <span>{{ article.meta.ts | date(format=\"%Y-%b-%d\") }}</span>
      <a href=\"/{{ article.meta.ts | date(format=\"%Y/%m/%d\") }}/{{ article.meta.link }}.html\">
      {{ article.meta.title }}
      </a>
    </li>
    {% endfor %}
    </ul>
  </div>
</body>
</html>
";
