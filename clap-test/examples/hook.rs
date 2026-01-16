use clap::{CommandFactory, FromArgMatches, Parser};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    mode: String,

    #[arg(long, hide = true)]
    hide: String,
}

fn main() {
    let mut cmd = <Cli as CommandFactory>::command();
    cmd = cmd.name("NameFromHook");

    let mut iter: Vec<_> = std::env::args_os().collect();
    iter.push("--hide".into());
    iter.push("hide_value_from_hook".into());

    let matches = cmd.try_get_matches_from(iter).unwrap_or_else(|e| e.exit());
    let cli = <Cli as FromArgMatches>::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    println!("cli mode:{}, hide:{}", cli.mode, cli.hide);
}
