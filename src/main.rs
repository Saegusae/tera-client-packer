mod cli;

use cli::{Cli, Commands, Parser};

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Pack {
      package_name,
      package_extension,
      package_size,
      output_dir,
      compress,
      workers,
      input_dir,
    } => unimplemented!(),
    Commands::Unpack {
      input_dir,
      manifest,
      workers,
      output_dir,
    } => unimplemented!(),
  }
}
