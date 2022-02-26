use clap::Parser;

// manda config view -> show config trong file config
// manda config add -> trigger flow add ssh key with host config
// manda ssh view -> show ssh key pub trong file config
// manda ssh create -> add ssh key

#[derive(Parser, Debug)]
struct Args {
    entity: String,
    command: String,
}
// Create a struct command
// Create README command
// Read/Write config file
fn main() {
    let args = Args::parse();
}
