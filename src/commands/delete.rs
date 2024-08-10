use crate::args::flags;
use anyhow::Error;

pub struct Delete;

impl Delete {
  pub fn new(delete_flags: &flags::DeleteFlags) -> Result<(), Error> {
    println!("{:?}", delete_flags);
    Ok(())
  }
}
