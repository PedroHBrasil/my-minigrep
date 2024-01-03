pub mod config;
mod search;

use std::fs;

pub fn run(config: config::Config) {
  let file_string = fs::read_to_string(config.file_path).expect("Couldn't load file.");
  let results = search::search(&config.search_string, &file_string, config.case_sensitive);
  println!("{:?}", results);
}
