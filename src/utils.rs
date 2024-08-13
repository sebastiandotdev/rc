use crate::models::{Env, Headers, Methods, RCConfig};
use anyhow::{bail, Context, Error as ErrorAnyhow, Result as ResultAnyhow};
use serde_json::from_str as from_json;
use std::io::{Error, Write};
use std::path::Path;
use std::{env, fs};

pub fn return_env_json(env: &str) -> Result<String, Error> {
  let method_names = ["GET", "POST", "DELETE", "PUT", "PATCH"];
  let methods = method_names
    .iter()
    .map(|&method| Methods::from_str(method).unwrap())
    .collect();

  let headers = Headers {
    authorization: String::from("Bearer <your-token>"),
    content_type: String::from("application/json"),
  };

  let config_rc = RCConfig {
    url: String::from("http://localhost:3000"),
    env: Env::from_str(env).unwrap(),
    methods,
    headers,
  };

  Ok(serde_json::to_string_pretty(&config_rc)?)
}

pub fn create_json_file(
  dir: &Path,
  filename: &str,
  value: &str,
) -> Result<(), Error> {
  let mut text = String::from(value);

  text.push('\n');
  create_file(dir, filename, &text)
}

pub fn create_file(
  dir: &Path,
  filename: &str,
  content: &str,
) -> Result<(), Error> {
  let path = dir.join(filename);

  if path.exists() {
    println!("✅ {}", "Skipped creating {filename} as it already exists");
    Ok(())
  } else {
    let mut file = fs::OpenOptions::new()
      .write(true)
      .create_new(true)
      .open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
  }
}

fn parse_config_file(path: &Path) -> ResultAnyhow<RCConfig> {
  let file_content = fs::read_to_string(path).with_context(|| {
    format!("Failed to read configuration file at {:?}", path)
  })?;

  let config: RCConfig = from_json(&file_content).with_context(|| {
    format!("Failed to parse JSON in configuration file at {:?}", path)
  })?;

  Ok(config)
}

pub fn read_config_file() -> Result<RCConfig, ErrorAnyhow> {
  let filename = "rc.config.json";

  if let Some(home_dir) = dirs::home_dir() {
    let global_path = home_dir.join(filename);

    if global_path.exists() {
      return parse_config_file(&global_path);
    }
  }

  if let Ok(cwd) = env::current_dir() {
    let local_path = cwd.join(filename);

    if local_path.exists() {
      return parse_config_file(&local_path);
    }
  }

  bail!(
    "Configuration file {} not found in any expected location.",
    filename
  )
}
