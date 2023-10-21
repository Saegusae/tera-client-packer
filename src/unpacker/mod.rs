mod test;

use crate::manifest::*;

use crossbeam::channel;
use flate2::read::GzDecoder;
use rayon::{ThreadPool, ThreadPoolBuilder};

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::thread;

pub struct Unpacker<'a> {
  worker_count: usize,
  workers: ThreadPool,

  manifest_path: &'a Path,
  input_dir: &'a Path,
  output_dir: &'a Path,
}

impl<'a> Default for Unpacker<'a> {
  fn default() -> Self {
    Self {
      worker_count: 0,
      workers: ThreadPoolBuilder::new().build().unwrap(),

      manifest_path: Path::new("packed/_manifest.json"),
      input_dir: Path::new("packed"),
      output_dir: Path::new("unpacked"),
    }
  }
}

impl<'a> Unpacker<'a> {
  pub fn new(manifest_path: &'a Path, input_dir: &'a Path, output_dir: &'a Path, worker_count: usize) -> Self {
    Self {
      worker_count,
      workers: ThreadPoolBuilder::new().num_threads(worker_count).build().unwrap(),

      manifest_path,
      input_dir,
      output_dir,
    }
  }

  pub fn unpack(&self) {
    let (tx, rx) = channel::bounded::<PackageEntry>(self.worker_count);
    let manifest = Manifest::from_file(self.manifest_path);

    let manager = thread::spawn(move || {
      for package in manifest.package_list {
        tx.send(package).unwrap();
      }
    });

    self.workers.broadcast(|_| {
      while let Ok(package) = rx.recv() {
        let file_path = self.input_dir.join(package.name);
        let file = File::open(file_path).unwrap();

        let mut decoder = GzDecoder::new(file);
        for entry in package.file_list {
          let mut buffer = vec![0u8; entry.size as usize];
          decoder.read_exact(&mut buffer).unwrap();

          let client_file_path = self.output_dir.join(entry.key);
          let parent_dir = client_file_path.parent().unwrap();

          if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).unwrap()
          }
          let mut output_file = File::create(client_file_path).unwrap();

          io::copy(&mut buffer.as_slice(), &mut output_file).unwrap();
        }
      }
    });

    manager.join().unwrap();
  }
}
