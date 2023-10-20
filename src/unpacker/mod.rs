mod test;

use crate::manifest::Manifest;

use crossbeam::channel;
use rayon::{ThreadPool, ThreadPoolBuilder};

use std::thread;

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
      workers: ThreadPoolBuilder::new().num_threads(worker_count).build().unwrap(),
    }
  }

  pub fn unpack(&self) {
    let (tx, rx) = channel::bounded::<String>(self.worker_count);

    let manager = thread::spawn(move || {});

    self.workers.broadcast(|_| while let Ok(_package_name) = rx.recv() {});

    manager.join().unwrap();
  }
}
