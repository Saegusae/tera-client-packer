mod test;

use crate::manifest::*;

use crossbeam::channel;
use flate2::read::GzDecoder;
use rayon::{ThreadPool, ThreadPoolBuilder};

use std::{fs::File, thread};

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
    let (tx, rx) = channel::bounded::<PackageEntry>(self.worker_count);
    let manifest = Manifest::from_file("./_manifest.json");

    let manager = thread::spawn(move || {
      for package in manifest.package_list {
        tx.send(package).unwrap();
      }
    });

    self.workers.broadcast(|_| {
      while let Ok(_package) = rx.recv() {
        let source = File::open(_package.name).unwrap();
      }
    });

    manager.join().unwrap();
  }
}
