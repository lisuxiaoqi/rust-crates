// Mix derive into builder
use clap::Args;
use clap::Command;
use clap::FromArgMatches;
use clap::Subcommand;

#[derive(Args, Debug)]
struct ThirdArgs {
    #[arg(long)]
    path: String,
}

#[derive(Subcommand, Debug)]
enum ThirdSubcommands {
    #[command(name = "tx")]
    Tx {
        #[arg(long)]
        hash: String,
    },
    Block {
        #[arg(long)]
        hash: String,
        #[arg(long)]
        height: u64,
    },
}

fn main() {
    //create empty command
    let cmd = Command::new("Cli")
        .name("cli")
        .about("Cli command from builder");

    //include args from derive
    let cmd = ThirdArgs::augment_args(cmd);

    //include args from subcommand
    let cmd = ThirdSubcommands::augment_subcommands(cmd);

    //get raw args from os
    let matches = cmd.get_matches();

    //populate into struct ThirdArgs
    let args = ThirdArgs::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    println!("ThirdArgs:{:#?}", args);

    //populate into struct ThirdSubcommands
    let cmds = ThirdSubcommands::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    println!("ThirdSubcommands:{cmds:#?}");
}
