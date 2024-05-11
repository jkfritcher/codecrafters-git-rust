use anyhow::Result;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{fs, io::{Read, Write}, str};

const GIT_OBJECTS_DIR: &str = ".git/objects";


// Reads an object from the git object store
// Takes the hash in binary form, e.g. [u8; 20]
pub fn read_object_bin(hash: &[u8]) -> Result<Vec<u8>> {
    read_object_hex(&hex::encode(hash))
}

// Reads an object from the git object store
// Takes the hash in hex encoded form, e.g. 40 character string
pub fn read_object_hex(hash: &str) -> Result<Vec<u8>> {
    // Build object path and name from hash
    let object_name = format!("{}/{}/{}",
                      GIT_OBJECTS_DIR, &hash[0..2].to_ascii_lowercase(), &hash[2..].to_ascii_lowercase()
    );

    // Open object and decode it into a Vec
    let obj = fs::File::open(object_name)?;
    let mut decoder = ZlibDecoder::new(obj);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;

    Ok(data)
}

pub fn write_object(data: &[u8]) -> Result<String> {
    // Calculate SHA1 hash of serialized data
    let hash = {
        let mut hasher = Sha1::new();
        hasher.update(&data);
        hasher.finalize()
    };
    let prefix = hex::encode(&hash[0..1]);
    let fname = hex::encode(&hash[1..]);

    // Create prefix directory, if needed
    fs::create_dir_all(format!("{}/{}", GIT_OBJECTS_DIR, prefix))?;

    // Open output file and compress object into it
    let obj = fs::File::create(format!("{}/{}/{}", GIT_OBJECTS_DIR, prefix, fname))?;
    let mut encoder = ZlibEncoder::new(obj, Compression::default());
    encoder.write_all(&data)?;
    encoder.finish()?;

    Ok(format!("{}{}", prefix, fname))
}

pub fn split_header(data: &[u8]) -> Result<Vec<&[u8]>> {
    let mut parts = Vec::with_capacity(2);
    let mut i = 0;
    while data[i] != b'\0' {
        i += 1;
    }
    parts.push(&data[0..i]);
    parts.push(&data[i+1..]);

    Ok(parts)
}

pub fn obj_type(data: &[u8]) -> Result<&str> {
    let mut i = 0;
    while data[i] != b' ' {
        i += 1;
    }
    Ok(str::from_utf8(&data[0..i])?)
}
