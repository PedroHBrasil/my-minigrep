use std::fs;

use my_minigrep::args::Args;
fn main() {
    let args = Args::get();
    let file_string = fs::read_to_string(args.file_path).expect("Couldn't load file.");
}
