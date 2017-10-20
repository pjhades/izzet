extern crate chrono;
extern crate clap;
extern crate izzet;
extern crate regex;
extern crate time;

use chrono::Local;
use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};
use izzet::error::{Error, Result};
use izzet::{gen, post};
use izzet::config::Config;
use regex::Regex;
use std::env;
use std::fs::{DirBuilder, OpenOptions};
use std::path::Path;
use std::io::Write;

// XXX this is ugly enough to deserve being rewritten
fn get_open_option(force: bool) -> OpenOptions {
    if force {
        OpenOptions::new().write(true).create(true).truncate(true).clone()
    }
    else {
        OpenOptions::new().write(true).create_new(true).clone()
    }
}

fn init_empty_site(m: &ArgMatches) -> Result<()> {
    let dir = Path::new(m.value_of("dir").unwrap_or("."));
    let opt = get_open_option(m.is_present("force"));

    for filename in &[izzet::CONFIG_FILE,
                      izzet::NOJEKYLL_FILE] {
        opt.open(dir.join(filename))
            .map_err(|e| format!("failed to create `{}`: {}", filename, e))?;
    }

    for dirname in &[izzet::FILES_DIR,
                     izzet::SRC_DIR,
                     izzet::TEMPLATES_DIR] {
        DirBuilder::new()
            .recursive(m.is_present("force"))
            .create(dir.join(dirname))
            .map_err(|e| format!("failed to create `{}`: {}", dirname, e))?;
    }

    for &(filename, html) in &[(izzet::INDEX_FILE, izzet::INDEX_HTML),
                               (izzet::POST_FILE, izzet::INDEX_HTML),
                               (izzet::ARCHIVE_FILE, izzet::ARCHIVE_HTML)] {
        let mut file = opt.open(dir.join(izzet::TEMPLATES_DIR).join(filename))
                          .map_err(|e| format!("failed to create `{}': {}", filename, e))?;
        file.write(html)?;
    }

    Ok(())
}

// XXX let's not put the timestamp in the markdown file title
// XXX we can simply create a Post with default value (empty)
//     and serialize it to the file
fn create_post(m: &ArgMatches) -> Result<()> {
    let link = m.value_of("link").expect("failed to get the link of post");
    if !Regex::new(r"^[A-Za-z0-9]+(-[A-Za-z0-9]+)*$")?.is_match(link) {
        return Err(Error::from_string(format!("invalid link name `{}'", link)));
    }

    let filename = format!("{}.md", link);
    let opt = get_open_option(m.is_present("force"));
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
    Path::new(izzet::CONFIG_FILE).exists()
}

fn generate_site(m: &ArgMatches) -> Result<()> {
    if !is_initialized() {
        return Err(Error::from_string("current directory is not initialized".to_string()));
    }

    let config = Config::from_path(Path::new(izzet::CONFIG_FILE))?;
    println!("config={:?}", config);
    gen::generate(config)?;

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
        ("gen",  Some(m)) => generate_site(m),
        ("post", Some(m)) => create_post(m),
        ("view", Some(m)) => Ok(()),
        _ => Ok(())
    };

    if let Err(e) = ret {
        println!("{}", e)
    }
}
