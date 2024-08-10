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

#[derive(Subcommand, Debug)]
pub enum RCSubcommands {
  Init(InitFlags),
  Get(GetFlags),
}
