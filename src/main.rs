use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

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
    } => unimplemented!(),
    Commands::Unpack {
      input_dir,
      manifest,
      output_dir,
    } => unimplemented!(),
    _ => println!("{:?}", args),
  }
}
