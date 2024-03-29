mod test;

use crate::manifest::*;

use crossbeam::channel;
use flate2::read::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{ThreadPool, ThreadPoolBuilder};
use sha1::{Digest, Sha1};
use tee::TeeReader;
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

  manifest_path: &'a Path,
  input_dir: &'a Path,
  output_dir: &'a Path,
}

impl<'a> Default for Packer<'a> {
  fn default() -> Self {
    Self {
      worker_count: 0,
      workers: ThreadPoolBuilder::new().build().unwrap(),

      package_name: String::from("client"),
      package_ext: String::from("cabx"),
      package_size: 500 * 1024_usize.pow(2),

      manifest_path: Path::new("packed/_manifest.json"),
      input_dir: Path::new("."),
      output_dir: Path::new("packed"),
    }
  }
}

impl<'a> Packer<'a> {
  pub fn new(
    input_dir: &'a Path,
    output_dir: &'a Path,
    manifest_path: &'a Path,
    package_name: String,
    package_ext: String,
    package_size: usize,
    worker_count: usize,
  ) -> Self {
    Self {
      worker_count,
      workers: ThreadPoolBuilder::new().num_threads(worker_count).build().unwrap(),

      package_name,
      package_ext,
      package_size,

      manifest_path,
      input_dir,
      output_dir,
    }
  }

  pub fn pack(&self) {
    let (tx, rx) = channel::bounded::<(i32, Vec<u8>)>(self.worker_count);

    let source_input = self.input_dir.to_owned();
    let package_size = self.package_size;

    let (package_name, package_ext) = (self.package_name.clone(), self.package_ext.clone());
    let manifest_path = self.manifest_path.to_owned();

    let manager = thread::spawn(move || {
      let mut sources = WalkDir::new(&source_input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .peekable();

      let rdr_prog = ProgressBar::new(0);

      let rdr_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
      rdr_prog.set_style(rdr_style);

      let mut manifest = Manifest::default();

      let mut buffer = Vec::<u8>::new();
      let mut package_index = 1;

      let mut file_list = Vec::<FileEntry>::new();

      let mut total_size: u64 = 0;
      let mut processed_bytes: usize = 0;
      let mut file_offset: u64 = 0;

      rdr_prog.set_prefix(format!("[Package {:03}]", package_index));

      while let Some(entry) = sources.next() {
        let path = entry.path();
        let key = path.strip_prefix(&source_input).unwrap().to_str().unwrap().to_string();

        rdr_prog.set_message(format!("{}", key));

        let mut file = File::open(path).unwrap();
        let bytes = file.read_to_end(&mut buffer).unwrap();

        rdr_prog.inc(1);

        file_list.push(FileEntry {
          key,
          offset: file_offset,
          size: bytes as u64,
        });

        processed_bytes += bytes;
        file_offset += bytes as u64 + 1;

        let next_file = sources.peek();
        if next_file.is_none() {
          rdr_prog.finish_with_message(format!(
            "Finished reading client files. Expected package count: {}",
            package_index
          ));
        }

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
          rdr_prog.set_prefix(format!("[Package {:03}]", package_index));

          package_index += 1;
        }
      }

      manifest.set_total_size(total_size).write(manifest_path);
    });

    self.workers.broadcast(|_| {
      while let Ok((idx, bytes)) = rx.recv() {
        let data: &[u8] = &*bytes;

        let output_path = self
          .output_dir
          .join(format!("{}.{:03}.{}", self.package_name, idx, self.package_ext));

        let encoder = GzEncoder::new(data, Compression::default());
        let mut hasher = Sha1::new();

        let mut tee = TeeReader::new(encoder, &mut hasher);
        let mut file = File::create(output_path).unwrap();

        let _bytes = io::copy(&mut tee, &mut file).unwrap();
        let hash = hasher.finalize();

        println!("Package {}: {:x}", idx, hash);
      }
    });

    manager.join().unwrap();
  }
}
