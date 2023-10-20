mod test;

use crate::manifest::*;

use crossbeam::channel;
use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::{ThreadPool, ThreadPoolBuilder};
use walkdir::WalkDir;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::thread;

pub struct Packer<'a> {
  worker_count: usize,
  workers: ThreadPool,

  package_name: String,
  package_ext: String,
  package_size: usize,

  input_dir: &'a Path,
  output_dir: &'a Path,
}

impl<'a> Default for Packer<'a> {
  fn default() -> Self {
    Self {
      input_dir: Path::new("."),
      output_dir: Path::new("packed"),
      package_name: String::from("client"),
      package_ext: String::from("cabx"),
      package_size: 500 * 1024_usize.pow(2),
      worker_count: 0,
      workers: ThreadPoolBuilder::new().build().unwrap(),
    }
  }
}

impl<'a> Packer<'a> {
  pub fn new(
    input_dir: &'a Path,
    output_dir: &'a Path,
    package_name: String,
    package_ext: String,
    package_size: usize,
    worker_count: usize,
  ) -> Self {
    Self {
      input_dir,
      output_dir,
      package_name,
      package_ext,
      package_size,
      worker_count,
      workers: ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap(),
    }
  }

  pub fn pack(&self) {
    let (tx, rx) = channel::bounded::<(i32, Vec<u8>)>(self.worker_count);

    let source_input = self.input_dir.to_owned();
    let package_size = self.package_size;

    let (package_name, package_ext) = (self.package_name.clone(), self.package_ext.clone());

    let manager = thread::spawn(move || {
      let mut sources = WalkDir::new(&source_input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .peekable();

      let mut manifest = Manifest::default();

      let mut buffer = Vec::<u8>::new();
      let mut package_index = 1;

      let mut file_list = Vec::<FileEntry>::new();

      let mut total_size: u64 = 0;
      let mut processed_bytes: usize = 0;
      let mut file_offset: u64 = 0;

      while let Some(entry) = sources.next() {
        let path = entry.path();
        let key = path
          .strip_prefix(&source_input)
          .unwrap()
          .to_str()
          .unwrap()
          .to_string();

        let mut file = File::open(path).unwrap();
        let bytes = file.read_to_end(&mut buffer).unwrap();

        file_list.push(FileEntry {
          key,
          offset: file_offset,
          size: bytes as u64,
        });

        processed_bytes += bytes;
        file_offset += bytes as u64 + 1;

        let next_file = sources.peek();
        if next_file.is_none() || processed_bytes >= package_size {
          let slice = buffer.clone();
          let packet = (package_index, slice);

          manifest.add_package(PackageEntry {
            name: format!("{}.{:03}.{}", package_name, package_index, package_ext),
            package_size: processed_bytes as u64,
            hash: None,
            file_list: file_list.clone(),
          });

          buffer.clear();
          file_list.clear();

          file_offset = 0;
          total_size += processed_bytes as u64;
          processed_bytes = 0;

          tx.send(packet).unwrap();
          package_index += 1;
        }
      }

      manifest
        .set_total_size(total_size)
        .write("./_manifest.json");
    });

    self.workers.broadcast(|_| {
      while let Ok((idx, bytes)) = rx.recv() {
        let mut data: &[u8] = &*bytes;

        let output_path = self.output_dir.join(format!(
          "{}.{:03}.{}",
          self.package_name, idx, self.package_ext
        ));

        let file = File::create(output_path).unwrap();
        let mut encoder = GzEncoder::new(file, Compression::default());

        io::copy(&mut data, &mut encoder).unwrap();
      }
    });

    manager.join().unwrap();
  }
}
