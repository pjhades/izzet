use error::Result;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn get_opener(force: bool) -> OpenOptions {
    let mut opener = OpenOptions::new();
    opener.write(true);
    if force {
        opener.create(true).truncate(true);
    }
    else {
        opener.create_new(true);
    }
    opener
}

pub fn fwrite<P: AsRef<Path>>(path: P, data: &[u8], force: bool) -> Result<()> {
    if let Some(dir) = path.as_ref().parent() {
        if !dir.exists() {
            create_dir_all(dir).map_err(|e| format!("error creating {:?}: {}", path.as_ref(), e))?;
        }
    }
    get_opener(force).open(&path)
        .and_then(|mut f| f.write(data))
        .map_err(|e| format!("error writing {:?}: {}", path.as_ref(), e))?;
    Ok(())
}
