use std::path::PathBuf;
use structopt::StructOpt;

/// Runs test on JSDoc documentation
/// Tests are run on code snippets in @example block
#[derive(StructOpt, Debug)]
#[structopt(name = "denodoc")]
pub struct Opt {
  // A flag, true if used in the command line. Note doc comment will
  // be used for the help message of the flag. The name of the
  // argument will be, by default, based on the name of the field.
  /// Activate debug mode
  #[structopt(short, long)]
  test: bool,

  /// Files to process
  #[structopt(name = "FILE", parse(from_os_str))]
  files: Vec<PathBuf>,
}
