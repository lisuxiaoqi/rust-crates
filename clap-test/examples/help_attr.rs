use clap::Parser;

#[derive(Parser)]
#[command(name = "HelpTest")]
#[command(version = "v0.0.1")]
#[command(about = "short help from attr")]
#[command(long_about = "long help from attr")]
struct Cli {}

fn main() {
    let _ = Cli::parse();
}
