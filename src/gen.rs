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

fn generate_posts(tera: &Tera, base: &Context, config: &Config, posts: &Vec<Post>) -> Result<()> {
    let out_dir = config.out_dir
                        .as_ref()
                        .map(|s| PathBuf::from(s))
                        .unwrap_or(env::current_dir()?);

    for post in posts {
        if post.meta.is_article {
            println!("rendering article {:?}", post.meta.link);
        }
        else {
            println!("rendering page {:?}", post.meta.link);
        }

        // XXX it would be better if tera supports editing context
        let mut ctx = Context::new();
        ctx.extend(base.clone());
        ctx.add("post", &post);

        let rendered = tera.render(::POST_FILE, &ctx)
            .map_err(|e| Error::new(format!("fail to render {:?}: {}", post.path, e)))?;

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

        write_file(filename, config, rendered.as_bytes())?;
    }

    Ok(())
}

pub fn generate(config: Config) -> Result<()> {
    let in_dir = config.in_dir
                       .as_ref()
                       .map(|s| PathBuf::from(s))
                       .unwrap_or(env::current_dir()?);

    // compile templates
    let tera = Tera::new(in_dir.join(::TEMPLATES_DIR)
                               .join("*")
                               .to_str()
                               .ok_or(Error::new("cannot get templates".to_string()))?)
        .map_err(|e| format!("compile templates fails: {}", e))?;

    // gather articles
    let mut articles = vec![];
    for entry in fs::read_dir(in_dir.join(::ARTICLES_DIR))? {
        articles.push(Post::from_file(&entry?.path())?);
    }
    articles.sort_by(|x, y| y.meta.ts.cmp(&x.meta.ts));

    // gather pages
    let mut pages = vec![];
    for entry in fs::read_dir(in_dir.join(::PAGES_DIR))? {
        pages.push(Post::from_file(&entry?.path())?);
    }
    pages.sort_by(|x, y| y.meta.ts.cmp(&x.meta.ts));

    // prepare context
    let mut ctx = Context::new();
    ctx.add("articles", &articles);
    ctx.add("pages", &pages);
    ctx.add("config", &config);

    // generate articles and pages
    generate_posts(&tera, &ctx, &config, &articles)?;
    generate_posts(&tera, &ctx, &config, &pages)?;

    Ok(())
}
