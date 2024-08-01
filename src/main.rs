mod args;
mod commands;
mod utils;

use args::flags::RCSubcommands;
use clap::Parser;
use commands::init::Init as InitCommand;

#[derive(Parser, Debug)]
#[command(name = "rc", version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub commands: RCSubcommands,
}

fn main() {
  let cli = Cli::parse();

  match &cli.commands {
    RCSubcommands::Init(init_flags) => {
      if let Err(err) = InitCommand::new(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
