use clap::Parser;

#[derive(Parser)]
/// short help info from doc
///
/// Long help info from doc after empty lines of short
struct Cli {}

fn main() {
    let _ = Cli::parse();
}
