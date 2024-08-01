use crate::args::flags::RCSubcommands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rc", version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub commands: RCSubcommands,
}

pub fn parse() -> Cli {
  Cli::parse()
}
