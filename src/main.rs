mod args;
mod commands;
mod utils;
mod cli;

use args::flags::RCSubcommands;
use commands::init::Init as InitCommand;

fn main() {
  let values = crate::cli::parse();

  match &values.commands {
    RCSubcommands::Init(init_flags) => {
      if let Err(err) = InitCommand::new(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
