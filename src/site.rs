use conf::Conf;
use error::{Error, Result, ResultContext};
use files;
use tera::{Tera, Context};
use post::{Post, PostKind};
use std::{env, fs};
use std::path::PathBuf;

pub struct Site {
    ctx: Context,
    articles: Vec<Post>,
    pages: Vec<Post>,
    tera: Tera,
}

impl Site {
    pub fn collect(conf: &Conf) -> Result<Self> {
        let in_dir = conf.in_dir.as_ref().map(PathBuf::from).unwrap_or(env::current_dir()?);

        let mut articles = vec![];
        let mut pages = vec![];
        let mut tera = Tera::new(in_dir.join(::THEME_DIR).join("*").to_str()
                                 .ok_or(Error::new("cannot get templates".to_string()))?)
            .context("compile templates fails".to_string())?;
        tera.autoescape_on(vec![]);

        for entry in fs::read_dir(in_dir.join(::SRC_DIR))? {
            let post = match Post::from_file(&entry?.path())? {
                None => continue,
                Some(p) => p,
            };
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
        ctx.add("conf", &conf);
        if let Some(p) = articles.first() {
            ctx.add("latest_article", p);
        }

        Ok(Site { ctx, articles, pages, tera })
    }

    pub fn generate(&self, conf: &Conf) -> Result<()> {
        let out_dir = conf.out_dir.as_ref().map(PathBuf::from).unwrap_or(env::current_dir()?);

        for p in self.articles.iter().chain(self.pages.iter()) {
            // skip the leading slash to make the output path correct
            let url = &p.url()?[1..];

            let mut ctx = Context::new();
            ctx.extend(self.ctx.clone());
            ctx.add("post", p);

            let rendered = self.tera.render(::POST_FILE, &ctx)
                .context(format!("fail to generate {}", url))?;
            println!("generating {}", url);
            files::fwrite(out_dir.join(url), rendered.as_bytes(),
                          conf.force.unwrap_or(false))?;
        }

        for f in &[::INDEX_FILE, ::ARCHIVE_FILE] {
            println!("generating {}", f);
            self.tera.render(f, &self.ctx)
                .context(format!("fail to generate {}", f))
                .and_then(|rendered| files::fwrite(out_dir.join(f), rendered.as_bytes(),
                                                   conf.force.unwrap_or(false)))?;
        }

        Ok(())
    }
}
