pub mod config;
mod search;

use std::fs;

/// Runs the search and displays the results based on the provided configuration.
/// 
/// # Arguments
/// 
/// * ˋconfigˋ - search configuration parameters
pub fn run(config: &config::Config) -> Result<(), &'static str> {
  let text = fs::read_to_string(&config.file_path).expect("Couldn't load file.");

  let results = search::search(&config.search_string, &text, config.case_sensitive);

  println!("{:?}", results);

  Ok(())
}
