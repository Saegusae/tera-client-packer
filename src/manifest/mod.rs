use serde::{Deserialize, Serialize};

use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
  pub total_size: u64,
  pub compressed: bool,
  pub parts: i32,
  pub package_list: Vec<PackageEntry>,
  pub revision: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageEntry {
  pub name: String,
  pub package_size: u64,
  pub hash: Option<String>,
  pub file_list: Vec<FileEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
  pub key: String,
  pub offset: u64,
  pub size: u64,
}

impl Default for Manifest {
  fn default() -> Self {
    Self {
      package_list: Vec::new(),
      compressed: true,
      parts: 0,
      revision: 0,
      total_size: 0,
    }
  }
}

impl Manifest {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
    let file = fs::File::open(path).unwrap();
    serde_json::from_reader::<fs::File, Self>(file).unwrap()
  }

  pub fn write<P: AsRef<Path>>(&self, path: P) {
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer(file, &self).unwrap();
  }

  pub fn set_total_size(&mut self, total_size: u64) -> &Self {
    self.total_size = total_size;
    self
  }

  pub fn add_package(&mut self, package: PackageEntry) {
    self.package_list.push(package);
    self.parts += 1;
  }
}
