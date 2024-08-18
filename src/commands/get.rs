use crate::args::flags;
use crate::utils::read_config_file;
use anyhow::{Context, Error};
use colored_json::prelude::*;
use reqwest::Client;
use serde_json::Value;

pub struct Get;

impl Get {
  pub async fn new(get_flags: &flags::GetFlags) -> Result<(), Error> {
    let client = Client::new();
    let rc_config = read_config_file()?;
    let base_url = rc_config.url;
    let url = match &get_flags.id {
      Some(id) => format!("{}/{}/{}", base_url, get_flags.path, id),
      None => format!("{}/{}", base_url, get_flags.path),
    };

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    let json: Value = serde_json::from_str(&body).context("Failed to parse JSON")?;

    let formatted_json = serde_json::to_string_pretty(&json)?.to_colored_json_auto()?;
    println!("{formatted_json}");

    Ok(())
  }
}
