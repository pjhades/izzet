extern crate chrono;
extern crate pulldown_cmark;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate tiny_http;
extern crate toml;

pub mod conf;
pub mod error;
pub mod files;
pub mod markdown;
pub mod new;
pub mod post;
pub mod server;
pub mod site;

pub const DEFAULT_PORT: u16 = 10950;

pub const CONFIG_FILE:   &str = "izzet.toml";
// XXX this should be made configurable
pub const NOJEKYLL_FILE: &str = ".nojekyll";
pub const INDEX_FILE:    &str = "index.html";
pub const POST_FILE:     &str = "post.html";
pub const ARCHIVE_FILE:  &str = "archive.html";

pub const SRC_DIR:       &str = "src";
pub const THEME_DIR:     &str = "theme";
