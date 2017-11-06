use chrono::Datelike;
use config::Config;
use error::{Error, Result};
use post::Post;
use std::env;
use std::fs::{self, DirBuilder};
use std::io::Write;
use tera::{Context, Tera};

fn output_post(post: &Post, config: &Config, content: &[u8]) -> Result<()> {
    let pwd = env::current_dir()?;
    let out_dir = config.out_dir
        .as_ref()
        .unwrap_or(&pwd);

    let dirname = format!("{}/{}/{}",
                          post.meta.ts.year(),
                          post.meta.ts.month(),
                          post.meta.ts.day());

    DirBuilder::new()
        .recursive(true)
        .create(&dirname)
        .map_err(|e| format!("fail to create {}: {}", &dirname, e))?;

    let mut filename = out_dir
        .join(&dirname)
        .join(&post.meta.link);

    if !filename.set_extension("html") {
        return Err(Error::new(format!("bad output filename {:?}", filename)));
    }

    let mut out = ::get_opener(config.force.unwrap_or(false))
        .open(&filename)
        .map_err(|e| format!("fail to create {:?}: {}", &filename, e))?;

    out.write(content)?;

    Ok(())
}

fn render_post(post: &Post, config: &Config, tera: &Tera) -> Result<String> {
    let mut ctx = Context::new();
    ctx.add("post", post);
    ctx.add("config", config);

    tera.render(::INDEX_FILE, &ctx)
        .map_err(|e| Error::new(format!("fail to render {:?}: {}", post.path, e)))
}

pub fn generate(config: Config) -> Result<()> {
    let pwd = env::current_dir()?;

    let in_dir = config.in_dir
        .as_ref()
        .unwrap_or(&pwd);

    let tera = Tera::new(in_dir.join(::TEMPLATES_DIR)
                               .join("*")
                               .to_str()
                               .ok_or(Error::new("cannot get templates".to_string()))?)
        .map_err(|e| format!("compile templates fails: {}", e))?;

    let mut latest_post = None;

    for entry in fs::read_dir(in_dir.join(::SRC_DIR))? {
        let entry = entry?;
        let post = Post::from_file(&entry.path())?;

        println!("rendering {:?}", &entry.path());

        render_post(&post, &config, &tera)
            .and_then(|c| output_post(&post, &config, c.as_bytes()))?;

        latest_post = match latest_post {
            None => Some(post),
            Some(p) => Some(if post.meta.ts > p.meta.ts { post } else { p })
        };
    }

    Ok(())
}
