use error::{Result, ResultContext};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
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

pub fn fread<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut content = vec![];
    File::open(&path).and_then(|mut f| f.read_to_end(&mut content))
        .context(format!("error opening file {:?}", path.as_ref()))?;
    Ok(content)
}

pub fn fwrite<P: AsRef<Path>>(path: P, data: &[u8], force: bool) -> Result<()> {
    if let Some(dir) = path.as_ref().parent() {
        if !dir.exists() {
            create_dir_all(dir).context(format!("error creating {:?}", path.as_ref()))?;
        }
    }
    get_opener(force).open(&path)
        .and_then(|mut f| f.write(data))
        .context(format!("error writing {:?}", path.as_ref()))?;
    Ok(())
}
