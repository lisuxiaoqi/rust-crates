use clap::{Args, Parser};

#[derive(Parser)]
struct Cli {
    //same group args are default unique
    #[arg(long, group = "g1")]
    arg1_g1: Option<String>,
    #[arg(long, group = "g1")]
    arg2_g1: Option<String>,

    #[command(flatten)]
    import: Tx,

    #[arg(short, requires = "tx")]
    config: Option<String>,
}

//group is automatically create for struct with Default:
//multiple is true and required is true
#[derive(Args)]
#[group(required = false, multiple = false)]
struct Tx {
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    hash: Option<String>,
}

fn main() {
    let cli = Cli::parse();
}
