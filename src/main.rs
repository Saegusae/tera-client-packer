mod cli;

use cli::{Cli, Commands, Parser};
use tera_client_packer::packer::Packer;

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Pack {
      package_name,
      package_extension,
      package_size,
      output_dir,
      workers,
      input_dir,
    } => Packer::new(
      &input_dir,
      &output_dir,
      package_name,
      package_extension,
      package_size * 1024_usize.pow(2),
      workers,
    )
    .pack(),
    Commands::Unpack {
      input_dir: _,
      manifest: _,
      workers: _,
      output_dir: _,
    } => unimplemented!(),
  }
}
