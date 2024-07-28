use clap::Parser;
use dirs;
use serde_json::{json, to_string_pretty, Value};
use std::io::{self, Error, Write};
use std::path::Path;
use std::{env, fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  dir: Option<String>,

  #[arg(short, long, default_value_t = false)]
  init: bool,

  #[arg(short, long, default_value_t = false)]
  local: bool,

  #[arg(short, long, default_value_t = false)]
  global: bool,
}

fn main() {
  let args = Args::parse();

  if let Err(err) = init_project(args) {
    eprintln!("Oh: {}", err)
  }
}

fn init_project(init_flags: Args) -> Result<(), std::io::Error> {
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

  if init_flags.init && init_flags.local {
    create_json_file(&dir_local, filename, &return_env_json("Local").unwrap())?;

    println!("✅ Local configuration file created");
  } else if init_flags.init && init_flags.global {
    create_json_file(&dir_global, filename, &return_env_json("Global").unwrap())?;

    println!("✅ Global configuration file created");
  } else {
    println!("❌ The locale of the file is not specified, please use the --local or --global option depending on your project type.")
  }

  Ok(())
}

fn return_env_json(env: &str) -> Result<serde_json::Value, io::Error> {
  let result = json!({
      "URL": "http://localhost:3000",
      "methods": ["GET", "POST", "DELETE", "PATCH"],
      "env": env
  });

  Ok(result)
}

fn create_json_file(
  dir: &Path,
  filename: &str,
  value: &Value,
) -> Result<(), Error> {
  let mut text = to_string_pretty(value)?;

  text.push('\n');

  create_file(dir, filename, &text)
}

fn create_file(dir: &Path, filename: &str, content: &str) -> Result<(), Error> {
  let path = dir.join(filename);

  if path.exists() {
    println!("ℹ️ {}", "Skipped creating {filename} as it already exists");
    Ok(())
  } else {
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(path)
      .expect("Failed to create {filename} file");

    file.write_all(content.as_bytes())?;
    Ok(())
  }
}
