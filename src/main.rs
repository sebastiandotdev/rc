use clap::Parser;
use log::info;
use serde_json::{json, to_string_pretty, Value};
use std::{env, fs};
use std::io::{Write, Error};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  dir: Option<String>,

  #[arg(short, long, default_value_t = false)]
  init: bool,
}

fn main() {
  let args = Args::parse();

  let _ = init_project(args);
}

fn init_project(init_flags: Args) -> Result<(), std::io::Error> {
  let cwd = env::current_dir().expect("Can't read current working directory.");

  let dir = if let Some(dir) = &init_flags.dir {
    let dir = cwd.join(dir);
    std::fs::create_dir_all(&dir)?;
    dir
  } else {
    cwd
  };

  if init_flags.init {
    create_json_file(
      &dir,
      "rc.config.json",
      &json!({
        "baseURL": "http://localhost:3000",
        "methods": ["GET", "POST", "DELETE", "PATCH"]
      }),
    )?;
    info!("✅ {}", "Project initialized");
  }
  Ok(())
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

fn create_file(
  dir: &Path,
  filename: &str,
  content: &str,
) -> Result<(), Error> {
  let path = dir.join(filename);

  if path.exists() {
    info!(
      "ℹ️ {}",
      format!("Skipped creating {filename} as it already exists")
    );
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
