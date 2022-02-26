
#![allow(unused)]

use clap::Parser;
use std::path::Path;

// manda config view -> show config trong file config
// manda config add -> trigger flow add ssh key with host config
// manda ssh view -> show ssh key pub trong file config
// manda ssh create -> add ssh key
#[derive(Parser, Debug)]
struct Cli {
    entity: String,
    command: String,
}

// Create a struct command
// Create README command
// Read/Write config file
struct SSHConfig { 
    host: String,
    host_name: String,
    user: String,
    identity_file: String
}

enum Command { 
    View,
    Add,
}

fn main() {
    let config_file_path = Path::new("/Users/kai/.ssh/config");
    let args = Cli::parse();
    let entity = &args.entity;
    let command = &args.command;
    println!("entity: {}, command: {}", &args.entity, &args.command);
    if entity == "config" {
        if command == "view" {
            let file_content = std::fs::read_to_string(config_file_path).expect("File not found");
            println!("With text:\n{}", file_content);
        } else if command == "add" {

        }
    } else if entity == "ssh" {
        
    }
}
