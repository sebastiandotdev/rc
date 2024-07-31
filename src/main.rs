mod args;
mod commands;
mod utils;

use args::flags::RCSubcommands;
use clap::Parser;
use commands::init;

#[derive(Parser, Debug)]
#[command(name = "rc", version, about = "REST Client is an easy-to-use command-line tool for interacting with RESTful APIs.", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub commands: RCSubcommands,
}

fn main() {
  let cli = Cli::parse();

  match &cli.commands {
    RCSubcommands::Init(init_flags) => {
      if let Err(err) = init::create_file_rc(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
