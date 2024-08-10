use anyhow::Error;
use crate::args::flags;

pub struct Patch;

impl Patch {
  pub fn new(patch_flags: &flags::PatchFlags) -> Result<(), Error> {
    println!("{:?}", patch_flags);
    Ok(())
  }
}
