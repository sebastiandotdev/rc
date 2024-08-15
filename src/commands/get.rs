use anyhow::Error;
use reqwest::Method;
use crate::args::flags;
use crate::http::http;

pub struct Get;

impl Get {
  pub async fn new(get_flags: &flags::GetFlags) -> Result<(), Error> {
    let fetch = http(Method::GET, &get_flags.path, None).await?;

    println!("GET Response: {:?}", fetch.text().await?);

    if let Some(id) = &get_flags.id {
      println!("O my is a id: {id}")
    }
    println!("{:?}", get_flags);
    Ok(())
  }
}
