use anyhow::{anyhow, Result};
use std::env;

mod commands;
use crate::commands::cmd_init;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Usage: your_bittorrent.sh <command>"));
    }
    let command = &args[1];

    match command.as_str() {
        // Usage: your_git.sh init
        "init" => {
            cmd_init()
        },
        _ => Err(anyhow!("Unknown command: {}", command))
    }
}