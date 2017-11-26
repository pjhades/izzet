use config::Config;
use error::{Error, Result};
use files;
use tera::{Tera, Context};
use post::{Post, PostKind};
use std::{env, fs};
use std::path::{Path, PathBuf};

pub struct Site {
    ctx: Context,
    articles: Vec<Post>,
    pages: Vec<Post>,
    tera: Tera,
}

impl Site {
    pub fn collect(config: &Config) -> Result<Self> {
        let in_dir = config.in_dir.as_ref().map(PathBuf::from).unwrap_or(env::current_dir()?);

        let mut articles = vec![];
        let mut pages = vec![];
        let mut tera = Tera::new(in_dir.join(::THEME_DIR).join("*").to_str()
                                 .ok_or(Error::new("cannot get templates".to_string()))?)
            .map_err(|e| format!("compile templates fails: {}", e))?;
        tera.autoescape_on(vec![]);

        for entry in fs::read_dir(in_dir.join(::SRC_DIR))? {
            let post = Post::from_file(&entry?.path())?;
            match post.kind {
                PostKind::Article => articles.push(post),
                PostKind::Page => pages.push(post),
            }
        }
        articles.sort_by(|x, y| y.ts.cmp(&x.ts));
        pages.sort_by(|x, y| y.ts.cmp(&x.ts));

        let mut ctx = Context::new();
        ctx.add("articles", &articles);
        ctx.add("pages", &pages);
        ctx.add("config", &config);
        if let Some(p) = articles.first() {
            ctx.add("latest_article", p);
        }

        Ok(Site { ctx, articles, pages, tera })
    }

    pub fn generate(&self, config: &Config) -> Result<()> {
        let out_dir = config.out_dir.as_ref().map(PathBuf::from).unwrap_or(env::current_dir()?);

        for p in self.articles.iter().chain(self.pages.iter()) {
            let mut path = match p.kind {
                PostKind::Article => PathBuf::from(p.ts.format("%Y/%m/%d").to_string()).join(&p.link),
                PostKind::Page => PathBuf::from(&p.link),
            };

            let mut ctx = Context::new();
            ctx.extend(self.ctx.clone());
            ctx.add("post", p);

            let rendered = self.tera.render(::POST_FILE, &ctx)
                .map_err(|e| Error::new(format!("fail to generate {:?}: {}", path, e)))?;
            path.set_extension("html");
            println!("generating {:?}", path);
            files::fwrite(out_dir.join(path), rendered.as_bytes(),
                          config.force.unwrap_or(false))?;
        }

        for f in &[::INDEX_FILE, ::ARCHIVE_FILE] {
            println!("generating {}", f);
            self.tera.render(f, &self.ctx)
                .map_err(|e| Error::new(format!("fail to generate {}: {}", f, e)))
                .and_then(|rendered| files::fwrite(out_dir.join(f), rendered.as_bytes(),
                                                   config.force.unwrap_or(false)))?;
        }

        Ok(())
    }
}
