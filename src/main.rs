extern crate chrono;
extern crate getopts;
extern crate izzet;
extern crate regex;
extern crate time;

use chrono::Local;
use getopts::{Matches, Options};
use izzet::error::{Error, Result};
use izzet::{gen, post};
use izzet::config::Config;
use regex::Regex;
use std::env;
use std::fs::{DirBuilder, OpenOptions};
use std::path::PathBuf;
use std::io::Write;

const PROG_NAME: &str = env!("CARGO_PKG_NAME");

fn get_open_option(m: &Matches) -> OpenOptions {
    let mut opener = OpenOptions::new();
    opener.write(true);
    match m.opt_present("force") {
        true => { opener.create(true).truncate(true); }
        false => { opener.create_new(true); }
    }

    opener
}

fn init_site(m: &Matches) -> Result<()> {
    let dir = m.free.get(1)
        .and_then(|s| Some(PathBuf::from(s)))
        .unwrap_or(env::current_dir()?);

    let opener = get_open_option(m);

    for filename in &[izzet::CONFIG_FILE,
                      izzet::NOJEKYLL_FILE] {
        opener.open(dir.join(filename))
              .map_err(|e| format!("failed to create `{}`: {}", filename, e))?;
    }

    for dirname in &[izzet::FILES_DIR,
                     izzet::SRC_DIR,
                     izzet::TEMPLATES_DIR] {
        DirBuilder::new()
            .recursive(m.opt_present("force"))
            .create(dir.join(dirname))
            .map_err(|e| format!("failed to create `{}`: {}", dirname, e))?;
    }

    for &(filename, html) in &[(izzet::INDEX_FILE, izzet::INDEX_HTML),
                               (izzet::POST_FILE, izzet::INDEX_HTML),
                               (izzet::ARCHIVE_FILE, izzet::ARCHIVE_HTML)] {
        let mut file = opener.open(dir.join(izzet::TEMPLATES_DIR)
                                      .join(filename))
                             .map_err(|e| format!("failed to create `{}': {}", filename, e))?;
        file.write(html)?;
    }

    Ok(())
}

// XXX let's not put the timestamp in the markdown file title
// XXX we can simply create a Post with default value (empty)
//     and serialize it to the file
fn create_post(m: &Matches) -> Result<()> {
    let link = m.free.get(1).expect("failed to get the link of post");
    if !Regex::new(r"^[A-Za-z0-9]+(-[A-Za-z0-9]+)*$")?.is_match(link) {
        return Err(Error::from_string(format!("invalid link name `{}'", link)));
    }

    let filename = format!("{}.md", link);
    let opt = get_open_option(m);
    let mut file = opt.open(&filename)
                      .map_err(|e| format!("failed to create `{}': {}",
                                           filename, e))?;

    file.write(format!("title = ''\n\
                        link = '{}'\n\
                        timestamp = '{:?}'\n\
                        {}\n",
                       link,
                       Local::now(),
                       post::METADATA_DELIM_LINE)
               .as_bytes())?;

    Ok(())
}

fn is_initialized() -> bool {
    PathBuf::from(izzet::CONFIG_FILE).exists()
}

fn generate_site(_: &Matches) -> Result<()> {
    if !is_initialized() {
        return Err(Error::from_string("current directory is not initialized".to_string()));
    }

    let config = Config::from_path(&PathBuf::from(izzet::CONFIG_FILE))?;
    gen::generate(config)?;

    Ok(())
}

fn usage(opts: &Options) {
    println!("{}", opts.usage(&format!("Usage: {} <options> <args>", PROG_NAME)));
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
            eprintln!("{}\nTry `{1} -h` or `{1} --help` to see the help.", e, PROG_NAME);
            return;
        },
    };

    if matches.opt_present("h") {
        usage(&opts);
        return;
    }

    if matches.opt_present("V") {
        println!("{} {}", PROG_NAME, env!("CARGO_PKG_VERSION"));
        return;
    }

    if !matches.opt_present("n")
        && !matches.opt_present("p")
        && !matches.opt_present("g") {
        println!("{}: nothing to do", PROG_NAME);
        return;
    }

    let ret = if matches.opt_present("n") {
        init_site(&matches)
    }
    else if matches.opt_present("p") {
        create_post(&matches)
    }
    else if matches.opt_present("g") {
        generate_site(&matches)
    }
    else {
        Ok(())
    };

    if let Err(e) = ret {
        println!("{}", e)
    }
}
