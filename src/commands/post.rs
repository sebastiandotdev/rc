use anyhow::Error;
use crate::args::flags;

pub struct Post;

impl Post {
  pub fn new(post_flags: &flags::PostFlags) -> Result<(), Error> {
    println!("{:?}", post_flags);
    Ok(())
  }
}
