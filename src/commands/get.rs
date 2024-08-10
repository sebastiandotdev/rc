use anyhow::Error;

use crate::args::flags;

pub struct Get;

impl Get {
  pub fn new(get_flags: &flags::GetFlags) -> Result<(), Error> {
    if let Some(id) = &get_flags.id {
      println!("O my is a id: {id}")
    }
    println!("{:?}", get_flags);
    Ok(())
  }
}
