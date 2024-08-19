use anyhow::{bail, Context, Error as AnyError};
use dirs;
use dirs::home_dir;
use std::env::current_dir;

use crate::args::flags;
use crate::utils::{create_json_file, return_env_json};

pub struct Init;

impl Init {
  pub fn new(init_flags: &flags::InitFlags) -> Result<(), AnyError> {
    let cwd = current_dir().context("Can't read current working directory.")?;
    let gwd = home_dir().context("❌ Can't determine home directory.: {:?}")?;

    let filename = "rc.config.json";

    let config = if init_flags.global {
      return_env_json("Global")
    } else {
      return_env_json("Local")
    };

    match config {
      Ok(config) => {
        let dir = if init_flags.global { &gwd } else { &cwd };
        let scope = if init_flags.global { "Global" } else { "Local" };

        create_json_file(dir, filename, &config)?;
        println!("✅ {} configuration file created", scope);

        Ok(())
      }
      Err(e) => bail!("❌ Failed to create configuration file: {:?}", e),
    }
  }
}
