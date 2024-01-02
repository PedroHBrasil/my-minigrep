pub mod args;

use std::fs;

pub fn run(args: args::Args) {
  let file_string = fs::read_to_string(args.file_path).expect("Couldn't load file.");
}
