use anyhow::{bail, Context, Error as AnyError};
use dirs;
use std::env;

use crate::{args::flags, utils};

pub struct Init;

impl Init {
  pub fn new(init_flags: &flags::InitFlags) -> Result<(), AnyError> {
    let cwd = env::current_dir().context("Can't read current working directory.")?;
    let global_dir = dirs::home_dir().context("❌ Can't determine home directory.: {:?}")?;

    let filename = "rc.config.json";

    let config = if init_flags.global {
      utils::return_env_json("Global")
    } else {
      utils::return_env_json("Local")
    };

    match config {
      Ok(config) => {
        let dir = if init_flags.global { &global_dir } else { &cwd };
        let scope = if init_flags.global { "Global" } else { "Local" };

        utils::create_json_file(dir, filename, &config)?;
        println!("✅ {} configuration file created", scope);

        Ok(())
      }
      Err(e) => {
        bail!("❌ Failed to create configuration file: {:?}", e)
      }
    }
  }
}
