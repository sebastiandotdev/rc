use crate::args::flags;
use anyhow::Error;

pub struct Patch;

impl Patch {
  pub fn new(patch_flags: &flags::PatchFlags) -> Result<(), Error> {
    println!("{:?}", patch_flags);
    Ok(())
  }
}
