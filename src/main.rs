extern crate getopts;
extern crate izzet;
extern crate toml;

use getopts::{Matches, Options};
use izzet::error::{Error, Result};
use izzet::post::{Post, POST_META_END};
use izzet::gen;
use izzet::get_opener;
use izzet::config::Config;
use std::env;
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::process;
use std::io::Write;

const PROG_NAME: &str = env!("CARGO_PKG_NAME");

fn create_site(m: &Matches) -> Result<()> {
    let dir = m.free.get(1)
        .and_then(|s| Some(PathBuf::from(s)))
        .unwrap_or(env::current_dir()?);

    let opener = get_opener(m.opt_present("force"));

    for filename in &[
        izzet::CONFIG_FILE,
        izzet::NOJEKYLL_FILE
    ] {
        opener.open(dir.join(filename))
              .map_err(|e| format!("fail to create {}: {}", filename, e))?;
    }

    for dirname in &[
        izzet::FILES_DIR,
        izzet::SRC_DIR,
        izzet::TEMPLATES_DIR
    ] {
        DirBuilder::new()
            .recursive(m.opt_present("force"))
            .create(dir.join(dirname))
            .map_err(|e| format!("fail to create {}: {}", dirname, e))?;
    }

    for &(filename, html) in &[
        (izzet::INDEX_FILE,   izzet::INDEX_HTML),
        (izzet::POST_FILE,    izzet::INDEX_HTML),
        (izzet::ARCHIVE_FILE, izzet::ARCHIVE_HTML)
    ] {
        let mut file = opener.open(dir.join(izzet::TEMPLATES_DIR)
                                      .join(filename))
                             .map_err(|e| format!("fail to create {}: {}", filename, e))?;
        file.write(html)?;
    }

    Ok(())
}

fn create_post(m: &Matches) -> Result<()> {
    let link = m.free.get(1)
        .ok_or(Error::new("fail to get the link of post".to_string()))?;

    let filename = format!("{}.md", link);
    let opener = get_opener(m.opt_present("force"));
    let mut file = opener.open(&filename)
                         .map_err(|e| format!("fail to create {}: {}", filename, e))?;

    let mut post = Post::new();
    post.meta.link = link.to_string();

    file.write(toml::to_string(&post.meta)?.as_bytes())?;
    file.write(POST_META_END.as_bytes())?;
    file.write(&post.content.as_bytes())?;

    Ok(())
}

fn generate_site(m: &Matches, config: Config) -> Result<()> {
    let in_dir = m.opt_str("input")
        .map(|s| PathBuf::from(s))
        .unwrap_or(env::current_dir()?);
    let out_dir = m.opt_str("output")
        .map(|s| PathBuf::from(s))
        .unwrap_or(env::current_dir()?);

    gen::generate(config, in_dir, out_dir, m.opt_present("force"))?;

    Ok(())
}

fn usage(opts: &Options) {
    println!("{}", opts.usage(&format!("usage: {} <options> <args>", PROG_NAME)));
}

fn run(m: Matches) -> Result<()> {
    if m.opt_present("new") {
        create_site(&m)?;
        return Ok(());
    }

    let config = Config::from_file(m.opt_str("c")
                                    .map(|p| PathBuf::from(p))
                                    .unwrap_or(env::current_dir()?)
                                    .join(izzet::CONFIG_FILE))?;

    if m.opt_present("post") {
        create_post(&m)?;
    }
    else if m.opt_present("gen") {
        generate_site(&m, config)?;
    }

    Ok(())
}

fn main() {
    let mut opts = Options::new();

    // one of these flags should be specified
    opts.optflag("n", "new", "Initialize an empty site at the given location.");
    opts.optflag("p", "post", "Create a post with the given permalink.");
    opts.optflag("g", "gen", "Generate site, can be used along with -i and -o \
                              to specify the input and output location.");
    opts.optflag("f", "force", "Overwrite existing files when creating posts, \
                                generating site output files, etc.");

    opts.optopt("c", "config", "Run with the given configuration file. By default \
                                configuration file will be looked for under the \
                                current directory.", "CONFIG");
    opts.optopt("i", "input", "Input site directory. Read input files from current \
                              directory by default.", "INPUT");
    opts.optopt("o", "output", "Output site directory. Write to current directory \
                               by default.", "OUTPUT");

    opts.optflag("h", "help", "Show this help message.");
    opts.optflag("V", "version", "Display version information.");

    let matches = match opts.parse(env::args()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}\ntry `{1} -h` or `{1} --help` to see the help.", e, PROG_NAME);
            process::exit(1);
        },
    };

    if matches.opt_present("help") {
        usage(&opts);
        return;
    }

    if matches.opt_present("version") {
        println!("{} {}", PROG_NAME, env!("CARGO_PKG_VERSION"));
        return;
    }

    if !matches.opt_present("new")
        && !matches.opt_present("p")
        && !matches.opt_present("g") {
        println!("nothing to do.");
        process::exit(1);
    }

    let mutex_opts = ["new", "post", "gen"];
    if mutex_opts.iter()
        .map(|o| matches.opt_present(o) as u32)
        .sum::<u32>() != 1 {
        eprintln!("only one of `-n', `-p' and `-g' could be specified");
        process::exit(1);
    }

    if let Err(e) = run(matches) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
