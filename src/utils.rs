use serde_json::{json, to_string_pretty, Value};
use std::fs;
use std::io::{Error, Write};
use std::path::Path;

pub fn return_env_json(env: &str) -> Result<serde_json::Value, Error> {
  let result = json!({
      "URL": "http://localhost:3000",
      "methods": ["GET", "POST", "DELETE", "PATCH"],
      "env": env
  });

  Ok(result)
}

pub fn create_json_file(
  dir: &Path,
  filename: &str,
  value: &Value,
) -> Result<(), Error> {
  let mut text = to_string_pretty(value)?;

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
