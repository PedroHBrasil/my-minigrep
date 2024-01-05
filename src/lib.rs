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

#[cfg(test)]
mod test {
  use std::path::PathBuf;
  use crate::config::Config;
  use super::*;

  #[test]
  fn run_ok() {
    let config = Config {
      search_string: String::from("the"),
      file_path: PathBuf::from("tests/fixtures/poem.txt"),
      case_sensitive: true,
    };

    let result = run(&config);

    assert!(result.is_ok());
  }

}
