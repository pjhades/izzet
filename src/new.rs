use conf::Conf;
use error::{Result, ResultContext};
use files;
use std::fs;
use std::path::PathBuf;
use toml;

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
  <h1><a href=\"/\">{{ conf.title }}</a></h1>
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
  <h1><a href=\"/\">{{ conf.title }}</a></h1>
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

pub fn create_site(dir: PathBuf, force: bool) -> Result<()> {
    if !dir.exists() {
        fs::create_dir_all(&dir).context(format!("error creating {:?}", dir))?;
    }

    let site_dirs = &[
        ::SRC_DIR,
        ::THEME_DIR,
    ];
    for d in site_dirs {
        let p = dir.join(d);
        fs::create_dir_all(&p).context(format!("error creating {:?}", p))?;
    }

    let conf: Conf = Conf::default();
    let conf = toml::to_string(&conf)?;
    files::fwrite(&dir.join(::CONFIG_FILE), conf.as_bytes(), force)?;

    let site_files = &[
        ::NOJEKYLL_FILE,
    ];
    for f in site_files {
        files::fwrite(&dir.join(f), &[], force)?;
    }

    let site_templates = &[
        (::POST_FILE,    POST_HTML),
        (::INDEX_FILE,   INDEX_HTML),
        (::ARCHIVE_FILE, ARCHIVE_HTML),
    ];
    for &(f, html) in site_templates {
        files::fwrite(&dir.join(::THEME_DIR).join(f), html, force)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::{env, fs};

    #[test]
    fn test_create_site() {
        let dir = env::temp_dir().join("new");
        create_site(dir.clone(), true).unwrap();
        assert!(dir.join(::CONFIG_FILE).exists());
        assert!(dir.join(::NOJEKYLL_FILE).exists());
        assert!(dir.join(::SRC_DIR).exists());
        assert!(dir.join(::THEME_DIR).exists());
        fs::remove_dir_all(dir).unwrap();
    }
}
