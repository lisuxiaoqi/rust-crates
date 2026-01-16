use clap::Parser;

#[derive(Parser)]
#[command(version, about)] //read value from cargo file as defaults
struct Cli {}

fn main() {
    let _ = Cli::parse();
}
