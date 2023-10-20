mod test;

use crate::manifest;

pub struct Packer {}

impl Default for Packer {
  fn default() -> Self {
    Self {}
  }
}

impl Packer {
  pub fn new() -> Self {
    Self {}
  }

  pub fn pack(&self) {}
}
