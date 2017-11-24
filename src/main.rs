extern crate getopts;
extern crate izzet;
extern crate toml;

use getopts::{Matches, Options};
use izzet::error::{Error, Result};
use izzet::{files, post, server};
use izzet::config::Config;
use izzet::site::Site;
use post::PostKind;
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process;

const PROG_NAME: &str = env!("CARGO_PKG_NAME");

fn create_site(m: &Matches) -> Result<()> {
    let force = m.opt_present("force");
    let dir = m.free.get(1).map(|s| PathBuf::from(s))
        .unwrap_or(env::current_dir()?);

    if !Path::new(&dir).exists() {
        create_dir_all(&dir).map_err(|e| format!("error creating {:?}: {}", &dir, e))?;
    }

    for d in izzet::SITE_DIRS {
        let p = dir.join(d);
        create_dir_all(&p)
            .map_err(|e| format!("error creating {:?}: {}", p, e))?;
    }

    let config: Config = Config::default();
    let config = toml::to_string(&config)?;
    files::fwrite(&dir.join(izzet::CONFIG_FILE), config.as_bytes(), force)?;

    for f in izzet::SITE_FILES {
        files::fwrite(&dir.join(f), &[], force)?;
    }

    for &(f, html) in izzet::SITE_TEMPLATES {
        files::fwrite(&dir.join(izzet::THEME_DIR).join(f), html, force)?;
    }

    Ok(())
}

fn usage(opts: &Options) {
    println!("{}", opts.usage(&format!("Usage: {} <options> <args>", PROG_NAME)));
}

fn run(m: Matches, action: &str) -> Result<()> {
    // Note that now we have no configuration file yet
    if action == "new" {
        create_site(&m)?;
        return Ok(());
    }

    // Load config file as a basis which may be overwritten
    // later by the command-line options.
    let mut config = Config::from_file(m.opt_str("config")
                                        .map(|p| PathBuf::from(p))
                                        .unwrap_or(env::current_dir()?)
                                        .join(izzet::CONFIG_FILE))?;

    if m.opt_present("force") {
        config.force = Some(true)
    }
    if let None = config.title {
        config.title = Some("Default title".to_string());
    }

    match action {
        "article" => {
            let link = m.free.get(1)
                .ok_or(Error::new("need the link of the article".to_string()))?;
            post::create_post(link.to_string(), config, PostKind::Article)?;
        },

        "page" => {
            let link = m.free.get(1)
                .ok_or(Error::new("need the link of the page".to_string()))?;
            post::create_post(link.clone(), config, PostKind::Page)?;
        },

        "gen" => {
            config.in_dir = m.opt_str("input");
            config.out_dir = m.opt_str("output");
            Site::collect(&config).and_then(|s| s.generate(&config))?;
        },

        "server" => {
            let dir = m.free.get(1)
                .map(PathBuf::from)
                .unwrap_or(env::current_dir()?);
            config.port = m.opt_str("listen")
                .and_then(|s| s.parse::<u16>().ok());
            server::forever(dir, config)?;
        },

        _ => {
            eprintln!("unknown action {}", action);
            process::exit(1);
        }
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
    opts.optflag("s", "server", "Start a local server to preview the generated site \
                                 specified by a directory.");
    opts.optflag("f", "force", "Overwrite existing files when creating articles, \
                                generating site output files, etc.");

    opts.optopt("c", "config", "Search for configuration file at the specified \
                                directory. By default the configuration file will be \
                                looked for under the current directory.", "CONFIG");
    opts.optopt("i", "input", "Input site directory. Read input files from current \
                              directory by default.", "INPUT");
    opts.optopt("o", "output", "Output site directory. Write to current directory \
                               by default.", "OUTPUT");
    opts.optopt("l", "listen", "Port on which the local server will listen.", "PORT");

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

    let mutex_opts = ["new", "article", "gen", "page", "server"];
    let action = mutex_opts
        .iter()
        .filter(|o| matches.opt_present(o))
        .collect::<Vec<_>>();

    match action.len() {
        0 => {
            println!("nothing to do");
            return;
        },
        1 => {
            if let Err(e) = run(matches, action.first().unwrap()) {
                eprintln!("{}", e);
                process::exit(1);
            }
        },
        _ => {
            eprintln!("only one of `-n', `-a', `-p', `-s' and `-g' could be specified");
            process::exit(1);
        }
    }
}
