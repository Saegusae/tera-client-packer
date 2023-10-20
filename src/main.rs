use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{self, Write},
  path::PathBuf,
};

pub mod packer;
pub mod unpacker;

#[derive(Debug, Parser)]
#[command(name = "tera-client-packer", author)]
#[command(about = "CLI Utility to pack TERA game client for distribution", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  #[command(arg_required_else_help = true)]
  Pack {
    #[arg(long, short = 'n', default_value = "client")]
    package_name: String,

    #[arg(long, short = 'e', default_value = "cabx")]
    package_extension: String,

    #[arg(long, short = 's', default_value_t = 500)]
    package_size: u64,

    #[arg(long, short, default_value = "packed")]
    output_dir: PathBuf,

    #[arg(long, short, default_value_t = false)]
    compress: bool,

    input_dir: PathBuf,
  },

  #[command(arg_required_else_help = true)]
  Unpack {
    #[arg(long, short)]
    input_dir: PathBuf,

    #[arg(long, short, default_value = "manifest.json")]
    manifest: PathBuf,

    output_dir: PathBuf,
  },
}

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
  total_size: u64,
  compressed: bool,
  parts: i32,
  package_list: Vec<PackageEntry>,
  revision: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageEntry {
  name: String,
  package_size: u64,
  hash: Option<String>,
  file_list: Vec<FileEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FileEntry {
  key: String,
  offset: u64,
  size: u64,
}

fn pack(
  input_dir: PathBuf,
  output_dir: PathBuf,
  name: &String,
  extension: &String,
  package_size: u64,
  _compress: bool,
) {
  if !output_dir.exists() {
    fs::create_dir_all(&output_dir).unwrap();
  }

  let sources = walkdir::WalkDir::new(&input_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|f| f.path().is_file())
    .collect::<Vec<walkdir::DirEntry>>();

  let progress = ProgressBar::new(sources.len() as u64);
  progress.set_style(
    ProgressStyle::with_template(
      "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] Packing: {pos}/{len}",
    )
    .unwrap()
    .progress_chars("#>-"),
  );

  let mut file_list = Vec::<FileEntry>::new();
  let mut package_list = Vec::<PackageEntry>::new();

  let mut total_size: u64 = 0;
  let mut processed_bytes = 0;
  let mut package_index = 1;

  let mut offset = 0;

  let mut package_file =
    fs::File::create(output_dir.join(format!("{name}.{:03}.{extension}", package_index))).unwrap();

  for input_file in sources {
    let file_path = input_file.path();
    let key = file_path
      .strip_prefix(&input_dir)
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let mut file = fs::File::open(file_path).unwrap();
    let size = io::copy(&mut file, &mut package_file).unwrap();
    package_file.flush().unwrap();

    processed_bytes += size;
    total_size += size;
    file_list.push(FileEntry { key, offset, size });

    offset += size + 1;
    progress.inc(1);

    if processed_bytes >= package_size * 1024_u64.pow(2) {
      package_list.push(PackageEntry {
        name: format!("{name}.{:03}.{extension}", package_index).to_string(),
        package_size: processed_bytes,
        hash: None,
        file_list: file_list.clone(),
      });

      file_list.clear();
      processed_bytes = 0;
      offset = 0;

      package_index += 1;
      package_file =
        fs::File::create(output_dir.join(format!("{name}.{:03}.{extension}", package_index)))
          .unwrap()
    }
  }

  let mut manifest_file = fs::File::create(output_dir.join("manifest.json")).unwrap();
  let manifest = Manifest {
    total_size,
    compressed: false,
    package_list,
    parts: package_index,
    revision: 1,
  };

  serde_json::to_writer(&mut manifest_file, &manifest).unwrap();
  manifest_file.flush().unwrap();
}

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Pack {
      package_name,
      package_extension,
      package_size,
      output_dir,
      compress,
      input_dir,
    } => pack(
      input_dir,
      output_dir,
      &package_name,
      &package_extension,
      package_size,
      compress,
    ),
    _ => println!("{:?}", args),
  }
}
