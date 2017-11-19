use config::Config;
use error::{Error, Result};
use post::Post;
use std::env;
use std::fs::{self, DirBuilder};
use std::io::Write;
use std::path::PathBuf;
use tera::{Context, Tera};

fn write_file(filename: PathBuf, config: &Config, content: &[u8]) -> Result<()> {
    let mut out = ::get_opener(config.force.unwrap_or(false))
        .open(&filename)
        .map_err(|e| format!("fail to create {:?}: {}", &filename, e))?;

    out.write(content)?;
    Ok(())
}

fn generate_post(tera: &Tera, base: &Context, config: &Config, post: &Post) -> Result<()> {
    let out_dir = config.out_dir
                        .as_ref()
                        .map(|s| PathBuf::from(s))
                        .unwrap_or(env::current_dir()?);
    let mut ctx = Context::new();
    ctx.extend(base.clone());
    ctx.add("post", post);

    let rendered = tera.render(::POST_FILE, &ctx)
        .map_err(|e| Error::new(format!("fail to generate {:?}: {}", post.path, e)))?;

    let mut filename = if post.meta.is_article {
        let dir = post.meta.ts.format("%Y/%m/%d").to_string();
        DirBuilder::new()
            .recursive(true)
            .create(&dir)
            .map_err(|e| format!("fail to create {}: {}", &dir, e))?;

        out_dir.join(&dir).join(&post.meta.link)
    }
    else {
        out_dir.join(&post.meta.link)
    };

    if !filename.set_extension("html") {
        return Err(Error::new(format!("bad output filename {:?}", filename)));
    }

    write_file(filename, config, rendered.as_bytes())
}

fn generate_site_page(tera: &Tera, ctx: &Context, config: &Config) -> Result<()> {
    let out_dir = config.out_dir
                        .as_ref()
                        .map(|s| PathBuf::from(s))
                        .unwrap_or(env::current_dir()?);

    for file in &[::INDEX_FILE, ::ARCHIVE_FILE] {
        let out_path = out_dir.as_path().join(&file);
        println!("generating {:?}", out_path);
        tera.render(file, &ctx)
            .map_err(|e| Error::new(format!("fail to generate {:?}: {}", out_path, e)))
            .and_then(|rendered| write_file(out_path, &config, rendered.as_bytes()))?;
    }

    Ok(())
}


pub fn generate(config: Config) -> Result<()> {
    let in_dir = config.in_dir
                       .as_ref()
                       .map(|s| PathBuf::from(s))
                       .unwrap_or(env::current_dir()?);

    let mut tera = Tera::new(in_dir.join(::THEME_DIR)
                                   .join("*")
                                   .to_str()
                                   .ok_or(Error::new("cannot get templates".to_string()))?)
        .map_err(|e| format!("compile templates fails: {}", e))?;
    // turn off auto escaping
    tera.autoescape_on(vec![]);

    let mut articles = vec![];
    let mut pages = vec![];
    for entry in fs::read_dir(in_dir.join(::SRC_DIR))? {
        let post = Post::from_file(&entry?.path())?;
        if post.meta.is_article {
            articles.push(post);
        }
        else {
            pages.push(post);
        }
    }
    articles.sort_by(|x, y| y.meta.ts.cmp(&x.meta.ts));
    pages.sort_by(|x, y| y.meta.ts.cmp(&x.meta.ts));

    let mut ctx = Context::new();
    ctx.add("articles", &articles);
    ctx.add("pages", &pages);
    ctx.add("config", &config);
    if let Some(p) = articles.first() {
        ctx.add("latest_article", p);
    }

    for post in articles.iter().chain(pages.iter()) {
        println!("generating {:?}", post.path);
        generate_post(&tera, &ctx, &config, &post)?;
    }

    generate_site_page(&tera, &ctx, &config)?;

    Ok(())
}
