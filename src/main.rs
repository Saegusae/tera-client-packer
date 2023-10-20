use clap::{Parser, Subcommand};

use std::path::PathBuf;

pub mod manifest;
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
  #[command(name = "pack", about = "Packs client files in given directory", long_about = None)]
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
  #[command(name = "unpack", about = "Unpacks client files to given directory", long_about = None)]
  Unpack {
    #[arg(long, short)]
    input_dir: PathBuf,

    #[arg(long, short, default_value = "manifest.json")]
    manifest: PathBuf,

    output_dir: PathBuf,
  },
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
