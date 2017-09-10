extern crate clap;

mod error;

use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};
use error::{Error, Result};
use std::env;
use std::fs;
use std::path;

fn init_empty_blog(m: &ArgMatches) -> Result<()> {
    let dir = path::Path::new(m.value_of("dir").unwrap_or("."));

    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dir.join(".config"))
        .map_err(|e| format!("failed to create `.config`: {}", e))?;

    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dir.join(".nojekyll"))
        .map_err(|e| format!("failed to create `.nojekyll`: {}", e))?;
    fs::create_dir(dir.join("files"))
        .map_err(|e| format!("failed to create `files`: {}", e))?;

    Ok(())
}

fn main() {
    let app = App::new(env::args().nth(0).unwrap())
        .settings(&[AppSettings::DisableVersion,
                    AppSettings::DeriveDisplayOrder,
                    AppSettings::UnifiedHelpMessage,
                    AppSettings::VersionlessSubcommands])
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize an empty blog")
                    .args(&[
                        Arg::from_usage("[dir] 'Directory for the blog'"),
                    ]))
        .subcommand(SubCommand::with_name("gen")
                    .about("Generate the blog"))
        .subcommand(SubCommand::with_name("view")
                    .about("View the blog locally"))
        .get_matches();

    let ret = match app.subcommand() {
        ("init", Some(m)) => init_empty_blog(m),
        ("gen", Some(m)) => Ok(()),
        ("view", Some(m)) => Ok(()),
        _ => Ok(())
    };

    if let Err(e) = ret {
        println!("{}", e)
    }
}
