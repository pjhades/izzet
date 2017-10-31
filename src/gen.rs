use chrono::Datelike;
use config::Config;
use error::Result;
use post::Post;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate(config: Config) -> Result<()> {
    let tera = Tera::new("templates/*")
               .map_err(|e| format!("compile templates failed: {}", e))?;

    for entry in fs::read_dir(::SRC_DIR)? {
        let entry = entry?;

        let post = Post::from_file(&entry.path())?;
        let mut ctx = Context::new();

        // XXX change this to const
        ctx.add("post", &post);
        ctx.add("config", &config);

        // XXX change this to const
        let s = tera.render("index.html", &ctx)
                    .map_err(|e| format!("render {:?} failed: {}", entry.path(), e))?;

        let dirname = format!("{}/{}/{}",
                              post.meta.timestamp.year(),
                              post.meta.timestamp.month(),
                              post.meta.timestamp.day());
        DirBuilder::new()
            .recursive(true)
            .create(&dirname)
            .map_err(|e| format!("failed to create {}: {}", &dirname, e))?;

        let filename = Path::new(&dirname).join(post.meta.link + ".html");
        let mut out = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename)
            .map_err(|e| format!("failed to create {:?}: {}", &filename, e))?;
        out.write(s.as_bytes())?;
    }

    Ok(())
}
