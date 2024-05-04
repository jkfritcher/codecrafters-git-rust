use anyhow::Result;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{fs, io::{ErrorKind, Read, Write}, str};

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

    let Object::Blob(content) = Object::deserialize(&data)?;
    print!("{}", str::from_utf8(&content)?);

    Ok(())
}

pub fn cmd_hash_object(file: &str) -> Result<()> {
    // Open and read the file
    let mut file = fs::File::open(file)?;
    let mut buf: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);
    file.read_to_end(&mut buf)?;
    drop(file);

    // Create Blob and serialize it
    let blob = Object::blob(buf);
    let data = blob.serialize();

    // Calculate SHA1 hash of serialized data
    let hash = {
        let mut hasher = Sha1::new();
        hasher.update(&data);
        hasher.finalize()
    };
    let prefix = hex::encode(&hash[0..1]);
    let fname = hex::encode(&hash[1..]);

    // Create prefix directory, if needed
    match fs::create_dir_all(format!("{}/{}", GIT_OBJECTS_DIR, prefix)) {
        Ok(()) => (),
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                return Err(err.into());
            }
            // Prefix already exists, ignore error
            ()
        }
    }

    // Open output file and compress object into it
    let obj = fs::File::create(format!("{}/{}/{}", GIT_OBJECTS_DIR, prefix, fname))?;
    let mut encoder = ZlibEncoder::new(obj, Compression::default());
    encoder.write_all(&data)?;
    encoder.finish()?;

    // Print object hash
    println!("{}", hex::encode(hash));

    Ok(())
}
