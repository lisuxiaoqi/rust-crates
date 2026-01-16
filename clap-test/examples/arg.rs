use clap::Args;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    //positional, no attribute, no name is required
    from: String,

    //nornal
    #[arg(short = 'n', long = "name")]
    name: String,

    //vec
    #[arg(short, long)]
    addr: Vec<String>,

    //flag
    #[arg(short, long)]
    verbose: bool,

    //optional
    #[arg(short, long)]
    record: Option<bool>,

    //default
    #[arg(short, long, default_value_t = String::from("male"))]
    gender: String,

    //args import
    #[command(flatten)]
    import: OutArgs,

    //command imports
    #[command(flatten)]
    flat: CommandImport,

    //change action
    #[arg(long, action=clap::ArgAction::Count)]
    action: u8,
}

#[derive(Args)]
struct OutArgs {
    #[arg(long)]
    /// arg1 from OutArgs
    out1: String,

    #[arg(help = "args2 from OutArgs")]
    #[arg(long)]
    out2: String,
}

#[derive(Parser, Clone)]
struct CommandImport {
    #[arg(long)]
    /// Optional arg_import
    arg_import: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    //show positional arg
    //Cargo run --example arg -- fromp
    println!("positional arg:{}", cli.from);

    //normal
    //Cargo run --example arg -- -n carol fromp
    println!("normal arg:{}", cli.name);

    //multiple vec
    //Cargo run --example arg -- -n carol fromp --addr addr1 --addr addr2
    println!("vec arg:{:?}", cli.addr);

    //verbose
    //Cargo run --example arg -- -n carol fromp --addr addr1 --addr addr2 --verbose
    println!("flag arg:{:?}", cli.verbose);

    //Cargo run --example arg -- -n carol fromp --addr addr1 --addr addr2 --verbose
    println!("optional arg:{:?}", cli.record);
    println!("default arg:{:?}", cli.gender);

    //Cargo run --example arg -- -n carol fromp --addr addr1 --addr addr2 --verbose --out1 o1 --out2 o2 --action --action
    println!("args action changed:{}", cli.action);
}
