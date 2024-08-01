use dirs;
use std::env;

use crate::{
  args::flags,
  utils::{create_json_file, return_env_json},
};

pub fn create_file_rc(
  init_flags: &flags::InitFlags,
) -> Result<(), std::io::Error> {
  let cwd = env::current_dir().expect("Can't read current working directory.");
  let global_dir = dirs::home_dir().expect("Can't determine home directory.");
  let filename = "rc.config.json";

  if init_flags.local {
    create_json_file(&cwd, filename, &return_env_json("Local").unwrap())?;

    println!("✅ Local configuration file created");
  } else if init_flags.global {
    create_json_file(
      &global_dir,
      filename,
      &return_env_json("Global").unwrap(),
    )?;

    println!("✅ Global configuration file created");
  } else {
    println!("❌ The locale of the file is not specified, please use the --local or --global option depending on your project type.")
  }

  Ok(())
}
