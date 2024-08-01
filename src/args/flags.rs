use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct InitFlags {
  #[arg(short, long, default_value_t = false)]
  pub local: bool,

  #[arg(short, long, default_value_t = false)]
  pub global: bool,
}

#[derive(Subcommand, Debug)]
pub enum RCSubcommands {
  Init(InitFlags),
}
