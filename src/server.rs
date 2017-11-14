use config::Config;
use error::{Error, Result};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

fn resp_with_status(req: Request, code: u16) -> Result<()> {
    println!("{} - {} {}", code, req.method().as_str(), req.url());
    req.respond(Response::empty(StatusCode(code)))
       .map_err(|e| Error::new(format!("fail to respond: {}", e)))
}

pub fn forever(dir: PathBuf, config: Config) -> Result<()> {
    let server = Server::http(("0.0.0.0", config.port.unwrap_or(::DEFAULT_PORT)))?;

    loop {
        let req = server.recv()?;
        if req.method() != &Method::Get {
            resp_with_status(req, 501)?;
            continue;
        }

        // skip the leading slash
        let mut path = dir.join(&req.url()[1..]);
        if path.is_dir() {
            path = path.join(::INDEX_FILE);
        }
        match File::open(&path) {
            Err(_) => resp_with_status(req, 404)?,
            Ok(f) => {
                println!("200 - {} {}", req.method().as_str(), req.url());
                let resp = Response::from_file(f)
                    .with_header(Header::from_str("Cache-Control: no-cache,no-store,must-revalidate")?);
                req.respond(resp)?;
            },
        }
    }
}
