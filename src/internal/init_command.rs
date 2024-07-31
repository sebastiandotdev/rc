use dirs;
use std::env;

use crate::{
  args::flags,
  utils::{create_json_file, return_env_json},
};

pub fn init_project(
  init_flags: &flags::InitFlags,
) -> Result<(), std::io::Error> {
  let cwd = env::current_dir().expect("Can't read current working directory.");
  let filename = "rc.config.json";

  let dir_local = if let Some(dir_local) = &init_flags.dir {
    let dir_local = cwd.join(dir_local);
    std::fs::create_dir_all(&dir_local)?;
    dir_local
  } else {
    cwd
  };

  let dir_global = dirs::home_dir().expect("Can't determine home directory.");

  if init_flags.local {
    create_json_file(&dir_local, filename, &return_env_json("Local").unwrap())?;

    println!("✅ Local configuration file created");
  } else if init_flags.global {
    create_json_file(
      &dir_global,
      filename,
      &return_env_json("Global").unwrap(),
    )?;

    println!("✅ Global configuration file created");
  } else {
    println!("❌ The locale of the file is not specified, please use the --local or --global option depending on your project type.")
  }

  Ok(())
}
