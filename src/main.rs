extern crate clap;

use clap::{App, AppSettings, SubCommand};
use std::env;

fn main() {
    let app = App::new(env::args().nth(0).unwrap())
        .settings(&[AppSettings::DisableVersion,
                    AppSettings::DeriveDisplayOrder,
                    AppSettings::UnifiedHelpMessage,
                    AppSettings::VersionlessSubcommands])
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize an empty blog"))
        .subcommand(SubCommand::with_name("gen")
                    .about("Generate the blog"))
        .subcommand(SubCommand::with_name("view")
                    .about("View the blog locally"))
        .get_matches();

    match app.subcommand_name() {
        Some("init") => println!("init"),
        Some("gen") => println!("gen"),
        Some("view") => println!("view"),
        _ => {}
    }
}
