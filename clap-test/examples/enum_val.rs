use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    First,
    Second,
}

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        Mode::First => {
            println!("First input get")
        }
        Mode::Second => {
            println!("Second input get")
        }
    }
}
