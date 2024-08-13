use anyhow::Error;

use crate::{args::flags, utils::read_config_file};

pub struct Get;

impl Get {
  pub fn new(get_flags: &flags::GetFlags) -> Result<(), Error> {
    let config_rc = read_config_file()?;

    println!("{:?}", config_rc);

    if let Some(id) = &get_flags.id {
      println!("O my is a id: {id}")
    }
    println!("{:?}", get_flags);
    Ok(())
  }
}
