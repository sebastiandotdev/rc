use crate::args::flags;
use anyhow::Error;

pub struct Put;

impl Put {
  pub fn new(put_flags: &flags::PutFlags) -> Result<(), Error> {
    println!("{:?}", put_flags);
    Ok(())
  }
}
