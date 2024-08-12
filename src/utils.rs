use crate::models::{Env, Methods, RCConfig};
use std::fs;
use std::io::{Error, Write};
use std::path::Path;

pub fn return_env_json(env: &str) -> Result<String, Error> {
  let method_names = ["GET", "POST", "DELETE", "PUT", "PATCH"];
  let methods = method_names
    .iter()
    .map(|&method| Methods::from_str(method).unwrap())
    .collect();

  let config_rc = RCConfig {
    url: String::from("http://localhost:3000"),
    env: Env::from_str(env).unwrap(),
    methods,
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
