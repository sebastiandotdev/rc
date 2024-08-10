mod args;
mod cli;
mod commands;
mod models;
mod utils;

use args::flags::RCSubcommands;
use commands::get::Get as GetCommand;
use commands::init::Init as InitCommand;

fn main() {
  let values = crate::cli::parse();

  match &values.commands {
    RCSubcommands::Init(init_flags) => {
      if let Err(err) = InitCommand::new(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Get(get_flags) => {
      if let Err(err) = GetCommand::new(get_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
