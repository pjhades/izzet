use config::Config;
use error::Result;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub fn forever(dir: PathBuf, config: Config) -> Result<()> {
    let server = Server::http(("0.0.0.0", config.port.unwrap_or(::DEFAULT_PORT)))?;

    loop {
        let req = server.recv()?;
        if req.method() != &Method::Get {
            req.respond(Response::empty(StatusCode(501)))?;
            continue;
        }

        // skip the leading slash
        let path = dir.join(&req.url()[1..]);
        match File::open(&path) {
            Err(e) => {
                eprintln!("fail to serve requested file {:?}: {}", &path, e);
                req.respond(Response::empty(StatusCode(404)))?;
            },
            Ok(f) => {
                let resp = Response::from_file(f)
                    .with_header(Header::from_str("Cache-Control: no-cache,no-store,must-revalidate")?);
                req.respond(resp)?;
            },
        }
    }
}
