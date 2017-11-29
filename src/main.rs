extern crate getopts;
extern crate izzet;
extern crate toml;

use getopts::{Matches, Options};
use izzet::error::{Error, Result};
use izzet::{files, post, server};
use izzet::conf::Conf;
use izzet::site::Site;
use post::PostKind;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

const PROG_NAME: &str = env!("CARGO_PKG_NAME");

fn create_site(m: &Matches) -> Result<()> {
    let force = m.opt_present("force");
    let dir = m.free.get(1).map(PathBuf::from).unwrap_or(env::current_dir()?);

    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("error creating {:?}: {}", dir, e))?;
    }

    for d in izzet::SITE_DIRS {
        let p = dir.join(d);
        fs::create_dir_all(&p).map_err(|e| format!("error creating {:?}: {}", p, e))?;
    }

    let conf: Conf = Conf::default();
    let conf = toml::to_string(&conf)?;
    files::fwrite(&dir.join(izzet::CONFIG_FILE), conf.as_bytes(), force)?;

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
    if action == "new" {
        create_site(&m)?;
        return Ok(());
    }

    let mut conf = Conf::from_file(m.opt_str("conf").unwrap_or(izzet::CONFIG_FILE.to_string()))?;

    if m.opt_present("force") {
        conf.force = Some(true)
    }
    if let None = conf.title {
        conf.title = Some("Default title".to_string());
    }

    match action {
        "article" => {
            let path = m.free.get(1).ok_or(Error::new("need specify path to the article".to_string()))?;
            post::create_post(path.to_string(), conf, PostKind::Article)?;
        },

        "page" => {
            let path = m.free.get(1).ok_or(Error::new("need specify path to the page".to_string()))?;
            post::create_post(path.to_string(), conf, PostKind::Page)?;
        },

        "gen" => {
            conf.in_dir = m.opt_str("input");
            conf.out_dir = m.opt_str("output");
            Site::collect(&conf).and_then(|s| s.generate(&conf))?;
        },

        "server" => {
            let dir = m.free.get(1).map(PathBuf::from).unwrap_or(env::current_dir()?);
            conf.port = m.opt_str("listen")
                .and_then(|s| s.parse::<u16>().ok());
            server::forever(dir, conf)?;
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

    opts.optopt("c", "conf", "Search for configuration file at the specified \
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
