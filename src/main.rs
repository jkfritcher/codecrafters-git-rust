use anyhow::{anyhow, Result};
use std::env;

mod commands;
mod types;

use crate::commands::{
    cmd_cat_file, cmd_hash_object, cmd_init
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Usage: your_git.sh <command>"));
    }
    let command = &args[1];

    match command.as_str() {
        // Usage: your_git.sh init
        "init" => {
            cmd_init()
        },
        // Usage: your_git.sh cat-file -p <object hash>
        "cat-file" => {
            if args.len() < 4 || args[2] != "-p" {
                return Err(anyhow!("Usage: your_git.sh cat-file -p <object hash>"));
            }
            cmd_cat_file(&args[3])
        },
        // Usage: your_git.sh hash-object -w <file name>
        "hash-object" => {
            if args.len() < 4 || args[2] != "-w" {
                return Err(anyhow!("Usage: your_git.sh hash-object -w <file name>"));
            }
            cmd_hash_object(&args[3])
        },
        _ => Err(anyhow!("Unknown command: {}", command))
    }
}