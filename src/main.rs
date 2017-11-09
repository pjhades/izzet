extern crate getopts;
extern crate izzet;
extern crate toml;

use getopts::{Matches, Options};
use izzet::error::{Error, Result};
use izzet::post;
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
        .map(|s| PathBuf::from(s))
        .unwrap_or(env::current_dir()?);

    let opener = get_opener(m.opt_present("force"));

    // create a default configuration file
    let config: Config = Default::default();
    let config = toml::to_string(&config)?;
    opener.open(dir.join(izzet::CONFIG_FILE))
          .and_then(|mut f| f.write(config.as_bytes()))
          .map_err(|e| format!("fail to create {}: {}", izzet::CONFIG_FILE, e))?;

    // create other files
    // XXX may need to do more things here later
    for filename in izzet::SITE_FILES {
        opener.open(dir.join(filename))
              .map_err(|e| format!("fail to create {}: {}", filename, e))?;
    }

    // create directories
    for dirname in izzet::SITE_DIRS {
        DirBuilder::new()
            .recursive(m.opt_present("force"))
            .create(dir.join(dirname))
            .map_err(|e| format!("fail to create {}: {}", dirname, e))?;
    }

    // create default templates
    for &(filename, html) in izzet::SITE_TEMPLATES {
        let mut file = opener.open(dir.join(izzet::TEMPLATES_DIR)
                                      .join(filename))
                             .map_err(|e| format!("fail to create {}: {}", filename, e))?;
        file.write(html)?;
    }

    Ok(())
}

fn usage(opts: &Options) {
    println!("{}", opts.usage(&format!("usage: {} <options> <args>", PROG_NAME)));
}

fn run(m: Matches) -> Result<()> {
    // Note that now we have no configuration file yet
    if m.opt_present("new") {
        create_site(&m)?;
        return Ok(());
    }

    // Load config file as a basis which may be overwritten
    // later by the command-line options.
    let mut config = Config::from_file(m.opt_str("config")
                                        .map(|p| PathBuf::from(p))
                                        .unwrap_or(env::current_dir()?)
                                        .join(izzet::CONFIG_FILE))?;

    config.force = Some(m.opt_present("force"));

    if m.opt_present("article") || m.opt_present("page") {
        let link = m.free.get(1)
            .ok_or(Error::new("fail to get the link of article".to_string()))?;
        post::create_post(link.clone(), config, m.opt_present("article"))?;
    }
    else if m.opt_present("gen") {
        config.in_dir = m.opt_str("input");
        config.out_dir = m.opt_str("output");
        gen::generate(config)?;
    }

    Ok(())
}

fn main() {
    let mut opts = Options::new();

    // One of these flags should be specified
    opts.optflag("n", "new", "Initialize an empty site at the given location.");
    opts.optflag("a", "article", "Create an article with the given permalink.");
    opts.optflag("p", "page", "Create a page with the given permalink.");
    opts.optflag("g", "gen", "Generate site, can be used along with -i and -o \
                              to specify the input and output location.");
    opts.optflag("f", "force", "Overwrite existing files when creating articles, \
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
        && !matches.opt_present("article")
        && !matches.opt_present("page")
        && !matches.opt_present("gen") {
        println!("nothing to do.");
        process::exit(1);
    }

    let mutex_opts = ["new", "article", "gen", "page"];

    match mutex_opts.iter()
                    .map(|o| matches.opt_present(o) as u32)
                    .sum::<u32>() {
        0 => {
            println!("nothing to do.");
            process::exit(1);
        },
        1 => (),
        _ => {
            eprintln!("only one of `-n', `-a', `-p' and `-g' could be specified");
            process::exit(1);
        }
    };

    if let Err(e) = run(matches) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
