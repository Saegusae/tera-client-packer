pub use clap::Parser;

use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "tera-client-packer", author)]
#[command(about = "CLI Utility to pack TERA game client for distribution", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(arg_required_else_help = true)]
  #[command(name = "pack", about = "Packs client files in given directory", long_about = None)]
  Pack {
    #[arg(long, short = 'n', default_value = "client")]
    package_name: String,

    #[arg(long, short = 'e', default_value = "cabx")]
    package_extension: String,

    #[arg(long, short = 's', default_value_t = 500)]
    package_size: usize,

    #[arg(name = "manifest", short, long, default_value = "packed/_manifest.json")]
    manifest_path: PathBuf,

    #[arg(long, short, default_value = "packed")]
    output_dir: PathBuf,

    #[arg(long, short, default_value_t = 8)]
    workers: usize,

    input_dir: PathBuf,
  },

  #[command(arg_required_else_help = true)]
  #[command(name = "unpack", about = "Unpacks client files to given directory", long_about = None)]
  Unpack {
    #[arg(name = "manifest", short, long, default_value = "packed/_manifest.json")]
    manifest_path: PathBuf,

    #[arg(long, short, default_value = "packed")]
    input_dir: PathBuf,

    #[arg(long, short, default_value_t = 8)]
    workers: usize,

    output_dir: PathBuf,
  },
}
