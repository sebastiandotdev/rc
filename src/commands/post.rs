use crate::args::flags;
use crate::utils::read_config_file;
use anyhow::{Context, Error};
use colored_json::prelude::*;
use reqwest::{
  header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde_json::{from_str, to_string_pretty, Value};

pub struct Post;

impl Post {
  pub async fn new(post_flags: &flags::PostFlags) -> Result<(), Error> {
    let client = Client::new();
    let rc_config = read_config_file()?;

    let base_url = rc_config.url;
    let url = match &post_flags.id {
      Some(id) => format!("{}/{}/{}", base_url, post_flags.path, id),
      None => format!("{}/{}", base_url, post_flags.path),
    };

    let mut headers = HeaderMap::new();

    let authorization = rc_config.headers.authorization;
    let content_type = rc_config.headers.content_type;

    headers.insert(AUTHORIZATION, HeaderValue::from_str(&authorization)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_str(&content_type)?);

    let response = client.post(&url).headers(headers).send().await?;
    let body = response.text().await?;
    let json: Value = from_str(&body).context("Failed to parse JSON")?;

    let formatted_json = to_string_pretty(&json)?.to_colored_json_auto()?;
    println!("{formatted_json}");
    Ok(())
  }
}
