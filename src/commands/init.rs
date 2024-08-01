use dirs;
use std::env;
use std::io::{Error as ErrorIO, ErrorKind};

use crate::{args::flags, utils};

pub struct Init;

impl Init {
  pub fn new(init_flags: &flags::InitFlags) -> Result<(), ErrorIO> {
    let cwd = env::current_dir().map_err(|e| {
      eprintln!("❌ Can't read current working directory: {:?}", e);
      ErrorIO::new(ErrorKind::NotFound, e)
    })?;

    let global_dir = dirs::home_dir().ok_or_else(|| {
      let e = "Can't determine home directory.";

      eprintln!("❌ Can't determine home directory.: {:?}", e);
      ErrorIO::new(ErrorKind::NotFound, e)
    })?;

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
        eprintln!("❌ Failed to create configuration file: {:?}", e);

        Err(ErrorIO::new(ErrorKind::Other, e))
      }
    }
  }
}
