extern crate clap;

mod error;

use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};
use error::{Error, Result};
use std::env;
use std::fs;
use std::path;

fn init_empty_site(m: &ArgMatches) -> Result<()> {
    let dir = path::Path::new(m.value_of("dir").unwrap_or("."));

    let opt = match m.is_present("force") {
        true  => fs::OpenOptions::new()
                 .write(true).create(true).truncate(true).clone(),
        false => fs::OpenOptions::new()
                 .write(true).create_new(true).clone(),
    };

    opt.open(dir.join(".config"))
       .map_err(|e| format!("failed to create `.config`: {}", e))?;
    opt.open(dir.join(".nojekyll"))
       .map_err(|e| format!("failed to create `.nojekyll`: {}", e))?;

    fs::DirBuilder::new()
        .recursive(m.is_present("force"))
        .create(dir.join("files"))
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
                    .about("Initialize an empty site")
                    .args(&[
                        Arg::from_usage("[dir] 'Directory for the site'"),
                        Arg::from_usage("-f, --force 'Overwrite existing \
                                        site metadata files'"),
                    ]))
        .subcommand(SubCommand::with_name("gen")
                    .about("Generate the site"))
        .subcommand(SubCommand::with_name("view")
                    .about("View the site locally"))
        .get_matches();

    let ret = match app.subcommand() {
        ("init", Some(m)) => init_empty_site(m),
        ("gen", Some(m)) => Ok(()),
        ("view", Some(m)) => Ok(()),
        _ => Ok(())
    };

    if let Err(e) = ret {
        println!("{}", e)
    }
}
