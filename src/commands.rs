use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::{fs, io::Read};

use crate::types::Object;

const GIT_OBJECTS_DIR: &str = ".git/objects";

pub fn cmd_init() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");

    Ok(())
}

pub fn cmd_cat_file(object: &str) -> Result<()> {
    // Build object path and name from hash
    let object_name = format!("{}/{}/{}", GIT_OBJECTS_DIR, &object[0..2], &object[2..]);

    // Open object and decode it into a Vec
    let obj = fs::File::open(object_name)?;
    let mut decoder = ZlibDecoder::new(obj);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    let Object::Blob(content) = Object::build(&data)?;
    print!("{}", String::from_utf8_lossy(&content));

    Ok(())
}
