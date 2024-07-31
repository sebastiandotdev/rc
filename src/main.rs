mod args;
mod internal;
mod utils;

use args::flags::RCSubcommands;
use clap::Parser;
use internal::init_command::init_project;

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
      if let Err(err) = init_project(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
