// Mix builder into derive
use clap::Arg;
use clap::ArgAction;
use clap::Args;
use clap::Command;
use clap::FromArgMatches;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    path: String,

    #[command(subcommand)]
    cmd: ThirdCommands,

    #[command(flatten)]
    arg: ThirdArgs,
}

#[derive(Debug)]
struct ThirdArgs {
    url: String,
}

//mannually impl Args trait instead of from derive
//populate command
impl Args for ThirdArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        Self::augment_args_for_update(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd.arg(Arg::new("url").long("url").action(clap::ArgAction::Set))
    }
}
//populate value into structure
impl FromArgMatches for ThirdArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let mut m = matches.clone();
        Self::from_arg_matches_mut(&mut m)
    }

    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        if let Some(url) = matches.remove_one::<String>("url") {
            Ok(Self { url: url })
        } else {
            Err(clap::Error::new(clap::error::ErrorKind::ValueValidation))
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        if let Some(url) = matches.get_one::<String>("url") {
            self.url = url.clone();
            Ok(())
        } else {
            Err(clap::Error::new(clap::error::ErrorKind::ValueValidation))
        }
    }
}

// impl the Subcommand trait instead of derive
#[derive(Debug)]
enum ThirdCommands {
    Tx { hash: String },
    Block { hash: String, height: u64 },
}

impl Subcommand for ThirdCommands {
    fn augment_subcommands(cmd: clap::Command) -> clap::Command {
        Self::augment_subcommands_for_update(cmd)
    }

    fn augment_subcommands_for_update(cmd: clap::Command) -> clap::Command {
        cmd.subcommand(Command::new("tx").arg(Arg::new("hash").long("hash").action(ArgAction::Set)))
            .subcommand(
                Command::new("block")
                    .arg(Arg::new("hash").long("hash"))
                    .arg(
                        Arg::new("height")
                            .long("height")
                            .value_parser(clap::value_parser!(u64)),
                    ),
            )
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "tx" | "block")
    }
}

impl FromArgMatches for ThirdCommands {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        match matches.subcommand() {
            Some(("tx", sub_match)) => Ok(Self::Tx {
                hash: sub_match
                    .get_one::<String>("hash")
                    .expect("hash is required")
                    .clone(),
            }),
            Some(("block", sub_match)) => Ok(Self::Block {
                hash: sub_match.get_one::<String>("hash").unwrap().clone(),
                height: *sub_match.get_one::<u64>("height").unwrap(),
            }),
            Some((_, _)) => Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidSubcommand,
                "Valid subcommands are `add` and `remove`",
            )),
            None => Err(clap::Error::raw(
                clap::error::ErrorKind::MissingSubcommand,
                "Valid subcommands are `add` and `remove`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        match matches.subcommand() {
            Some(("tx", sub_match)) => {
                *self = Self::Tx {
                    hash: sub_match
                        .get_one::<String>("hash")
                        .expect("hash is required")
                        .clone(),
                }
            }
            Some(("block", sub_match)) => {
                *self = Self::Block {
                    hash: sub_match.get_one::<String>("hash").unwrap().clone(),
                    height: sub_match.get_one::<u64>("height").unwrap().clone(),
                }
            }
            Some((_, _)) => {
                return Err(clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `add` and `remove`",
                ))
            }
            None => (),
        }
        Ok(())
    }
}
fn main() {
    let cli = Cli::parse();
    println!("cli parsed:{cli:#?}");
}
