extern crate clap;
extern crate regex;
extern crate time;

mod error;
mod post;

use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};
use error::{Error, Result};
use regex::Regex;
use std::{env, fs, path};
use std::io::Write;

const CONFIG_FILENAME: &str = ".izzetconfig";
const NOJEKYLL_FILENAME: &str = ".nojekyll";
const FILES_DIRNAME: &str = "files";

fn get_open_option(force: bool) -> fs::OpenOptions {
    if force {
        fs::OpenOptions::new().write(true).create(true).truncate(true).clone()
    }
    else {
        fs::OpenOptions::new().write(true).create_new(true).clone()
    }
}

fn init_empty_site(m: &ArgMatches) -> Result<()> {
    let dir = path::Path::new(m.value_of("dir").unwrap_or("."));
    let opt = get_open_option(m.is_present("force"));

    opt.open(dir.join(CONFIG_FILENAME))
       .map_err(|e| format!("failed to create `{}`: {}", CONFIG_FILENAME, e))?;
    opt.open(dir.join(NOJEKYLL_FILENAME))
       .map_err(|e| format!("failed to create `{}`: {}", NOJEKYLL_FILENAME, e))?;

    fs::DirBuilder::new()
        .recursive(m.is_present("force"))
        .create(dir.join(FILES_DIRNAME))
        .map_err(|e| format!("failed to create `{}`: {}", FILES_DIRNAME, e))?;

    Ok(())
}

fn create_post(m: &ArgMatches) -> Result<()> {
    let link = m.value_of("link").expect("failed to get the link of post");
    if !Regex::new(r"^[A-Za-z0-9]+(-[A-Za-z0-9]+)*$")?.is_match(link) {
        return Err(Error::from_string(format!("invalid link name `{}'", link)));
    }

    let now = time::now();

    let filename = format!("{}-{}.md", time::strftime("%Y-%m-%d", &now)?, link);
    let opt = get_open_option(m.is_present("force"));
    let mut file = opt.open(&filename)
                      .map_err(|e| format!("failed to create `{}': {}",
                                           filename, e))?;

    file.write(format!("%%\ntitle =\nlink = {}\ntimestamp = {}\n%%\n",
                       link, time::strftime("%Y-%m-%d %H:%M:%S", &now)?)
               .as_bytes())?;

    Ok(())
}

fn main() {
    let app = App::new(env::args().nth(0).unwrap())
        .settings(&[AppSettings::DisableVersion,
                    AppSettings::DeriveDisplayOrder,
                    AppSettings::UnifiedHelpMessage,
                    AppSettings::VersionlessSubcommands])
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize an empty site")
                    .args(&[
                        Arg::from_usage("[dir] 'Directory for the site'"),
                        Arg::from_usage("-f, --force 'Overwrite existing \
                                        site metadata files'")
                    ]))
        .subcommand(SubCommand::with_name("post")
                    .about("Create a new post with the specified link name")
                    .args(&[
                        Arg::from_usage("<link> 'Link name of the post which'"),
                        Arg::from_usage("-f, --force 'Overwrite existing post'")
                    ]))
        .subcommand(SubCommand::with_name("gen")
                    .about("Generate the site"))
        .subcommand(SubCommand::with_name("view")
                    .about("View the site locally"))
        .get_matches();

    let ret = match app.subcommand() {
        ("init", Some(m)) => init_empty_site(m),
        ("gen",  Some(m)) => Ok(()),
        ("post", Some(m)) => create_post(m),
        ("view", Some(m)) => Ok(()),
        _ => Ok(())
    };

    if let Err(e) = ret {
        println!("{}", e)
    }
}
