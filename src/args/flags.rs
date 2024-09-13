use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct InitFlags {
  /// Generate configuration file in the root your of computer
  #[arg(short, long, default_value_t = false)]
  pub global: bool,
}

#[derive(Parser, Debug)]
pub struct GetFlags {
  pub path: String,

  #[arg(short, long)]
  pub id: Option<String>,

  #[arg(short, long)]
  pub query: Option<String>,
}

#[derive(Debug, Parser)]
pub struct PostFlags {
  pub path: String,

  #[arg(short, long)]
  pub id: Option<String>,

  #[arg(short, long)]
  pub query: Option<String>,
}

#[derive(Debug, Parser)]
pub struct DeleteFlags {}

#[derive(Debug, Parser)]
pub struct PatchFlags {}

#[derive(Debug, Parser)]
pub struct PutFlags {}

#[derive(Subcommand, Debug)]
pub enum RCSubcommands {
  #[command(about = "Initialize a new configuration file")]
  Init(InitFlags),

  #[command(about = "Make an http GET request to an external or local server")]
  Get(GetFlags),

  #[command(about = "Make an http POST request to an external or local server")]
  Post(PostFlags),

  #[command(about = "Make an http DELETE request to an external or local server")]
  Delete(DeleteFlags),

  #[command(about = "Make an http PUT request to an external or local server")]
  Put(PutFlags),

  #[command(about = "Make an http PATCH request to an external or local server")]
  Patch(PatchFlags),
}
