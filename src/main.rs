mod cli;

use cli::{Cli, Commands, Parser};
use tera_client_packer::{packer::Packer, unpacker::Unpacker};

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Pack {
      package_name,
      package_extension,
      package_size,
      manifest_path,
      output_dir,
      workers,
      input_dir,
    } => Packer::new(
      &input_dir,
      &output_dir,
      &manifest_path,
      package_name,
      package_extension,
      package_size * 1024_usize.pow(2),
      workers,
    )
    .pack(),
    Commands::Unpack {
      manifest_path,
      input_dir,
      output_dir,
      workers,
    } => Unpacker::new(&manifest_path, &input_dir, &output_dir, workers).unpack(),
  }
}
