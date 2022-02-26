
#![allow(unused)]

use clap::Parser;

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
enum Command { 
    View,
    Add,
}
fn main() {
    let args = Cli::parse();
    println!("entity: {}", &args.entity);
    println!("command: {}", &args.command);

}
