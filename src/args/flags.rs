use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct InitFlags {
  #[arg(short, long, default_value_t = false)]
  pub global: bool,
}

#[derive(Parser, Debug)]
pub struct GetFlags {
  path: String,

  #[arg(short, long)]
  pub id: Option<String>,
}

#[derive(Debug, Parser)]
pub struct PostFlags {}

#[derive(Debug, Parser)]
pub struct DeleteFlags {}

#[derive(Debug, Parser)]
pub struct PatchFlags {}

#[derive(Debug, Parser)]
pub struct PutFlags {}

#[derive(Subcommand, Debug)]
pub enum RCSubcommands {
  Init(InitFlags),
  Get(GetFlags),
  Post(PostFlags),
  Delete(DeleteFlags),
  Patch(PatchFlags),
  Put(PutFlags),
}
