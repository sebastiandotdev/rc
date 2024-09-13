mod args;
mod commands;
mod models;
mod utils;

use args::flags::RCSubcommands;
use clap::Parser;
use commands::delete::Delete as DeleteCommand;
use commands::get::Get as GetCommand;
use commands::init::Init as InitCommand;
use commands::patch::Patch as PatchCommand;
use commands::post::Post as PostCommand;
use commands::put::Put as PutCommand;

#[derive(Parser, Debug)]
#[command(name = "rc", version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub commands: RCSubcommands,
}

pub fn parse() -> Cli {
  Cli::parse()
}
#[tokio::main]
async fn main() {
  let values = parse();

  match &values.commands {
    RCSubcommands::Init(init_flags) => {
      if let Err(err) = InitCommand::new(init_flags) {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Get(get_flags) => {
      if let Err(err) = GetCommand::new(get_flags).await {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Post(post_flags) => {
      if let Err(err) = PostCommand::new(post_flags).await {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Delete(delete_flags) => {
      if let Err(err) = DeleteCommand::new(delete_flags) {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Patch(patch_flags) => {
      if let Err(err) = PatchCommand::new(patch_flags) {
        eprintln!("Oh: {}", err);
      }
    }

    RCSubcommands::Put(patch_flags) => {
      if let Err(err) = PutCommand::new(patch_flags) {
        eprintln!("Oh: {}", err);
      }
    }
  }
}
