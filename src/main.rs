use cli::{Cli, Commands, Parser};

mod cli;

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
  }
}
