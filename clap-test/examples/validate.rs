use clap::{CommandFactory, Parser};

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    #[arg(value_parser=len_check)]
    /// customize key length check
    key: String,

    #[arg(help = "positional arg with help")]
    #[arg(value_parser=clap::value_parser!(i32).range(1..10))]
    value: i32,
}

fn len_check(key: &str) -> Result<String, String> {
    if key.len() > 3 {
        Err("Length longer thant 3".into())
    } else {
        Ok(key.into())
    }
}

fn main() {
    let cli = Cli::parse();

    //do final mannual check here
    let v = cli.value;
    if v != 2 {
        let mut cmd = Cli::command();
        cmd.error(
            clap::error::ErrorKind::InvalidValue,
            "final check from app logic",
        )
        .exit();
    }
}
