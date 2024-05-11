use anyhow::{anyhow, Result};
use std::{fs, io::Read, str};

use crate::util::{obj_type, read_object_bin, read_object_hex, write_object};
use crate::types::Object;

pub fn cmd_init() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");

    Ok(())
}

pub fn cmd_cat_file(hash: &str) -> Result<()> {
    let data = read_object_hex(hash)?;

    let content = Object::deserialize(&data)?;
    match content {
        Object::Blob(content) => {
            print!("{}", str::from_utf8(&content)?);
            Ok(())
        },
        _ => Err(anyhow!("Unsupported object type.")),
    }
}

pub fn cmd_hash_object(file: &str) -> Result<()> {
    // Open and read the file
    let mut file = fs::File::open(file)?;
    let mut buf: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize);
    file.read_to_end(&mut buf)?;
    drop(file);

    // Create Blob and serialize it
    let blob = Object::blob(buf)?;
    let data = blob.serialize();

    // Write object and get the object hash
    let hash = write_object(&data)?;

    // Print object hash
    println!("{}", hash);

    Ok(())
}

pub fn cmd_ls_tree(hash: &str, name_only: bool) -> Result<()> {
    let data = read_object_hex(hash)?;
    let tree = Object::deserialize(&data)?;
    match tree {
        Object::Tree(entries) => {
            for entry in entries {
                if !name_only {
                    let obj = read_object_bin(entry.hash())?;
                    let obj_type = obj_type(&obj)?;
                    println!("{:0>6} {} {}\t{}", entry.mode(), obj_type, hex::encode(entry.hash()), entry.name())
                } else {
                    println!("{}", entry.name())
                }
            }
            Ok(())
        },
        o @ _ => Err(anyhow!("Expected a Tree object, and received a '{}'", o.name()))
    }
}
