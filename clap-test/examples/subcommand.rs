use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
/// Command entry for Cli
struct Cli {
    #[arg(short, long)]
    path: String,

    //flag another command
    #[command(flatten)]
    flat: CommandImport,

    #[command(subcommand)]
    command: CliSubCommand,
}

#[derive(Parser, Clone)]
struct CommandImport {
    #[arg(long)]
    arg_import: Option<String>,

    #[command(subcommand)]
    command: ImportSubCommand,
}

#[derive(Subcommand, Clone)]
#[command(about = "Import SubCommand")]
enum ImportSubCommand {
    /// cmd3 help in doc
    Cmd3 {
        #[arg(short, long)]
        /// key for cmd3
        key: String,
        #[arg(short, long)]
        /// value for cmd3
        value: String,
    },
}

#[derive(Subcommand)]
#[command(about = "CliSubCommand")]
enum CliSubCommand {
    /// cmd1 help in doc
    Cmd1 {
        #[arg(short, long)]
        /// key for cmd1
        key: String,
        #[arg(short, long)]
        /// value for cmd1
        value: String,
    },

    Cmd2(Cmd2Arg),
}

#[derive(Args)]
#[command(about = "Cmd2Arg help in about")]
struct Cmd2Arg {
    #[arg(short, long)]
    /// key for cmd2
    key: String,
    #[arg(short, long)]
    /// value for cmd2
    value: String,

    #[command(flatten)]
    import: ArgsImport,
}

#[derive(Args)]
struct ArgsImport {
    #[arg(long)]
    #[arg(help = "args to import")]
    import: Option<String>,
}

fn main() {
    let c = Cli::parse();
    match c.command {
        CliSubCommand::Cmd1 { key, value } => {
            println!("Get input from Cmd1:{key}, {value}");
        }
        CliSubCommand::Cmd2(Cmd2Arg { key, value, .. }) => {
            println!("Get input from Cmd2:{key}, {value}");
        }
    }
}
