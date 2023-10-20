mod test;

use crate::manifest;

use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct Unpacker {
  worker_count: usize,
  workers: ThreadPool,
}

impl Default for Unpacker {
  fn default() -> Self {
    Self {
      worker_count: 0,
      workers: ThreadPoolBuilder::new().build().unwrap(),
    }
  }
}

impl Unpacker {
  pub fn new(worker_count: usize) -> Self {
    Self {
      worker_count,
      workers: ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap(),
    }
  }

  pub fn unpack(&self) {}
}
