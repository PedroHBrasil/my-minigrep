pub mod config;
mod search;

use std::fs;

/// Runs the search and displays the results based on the provided configuration.
/// 
/// # Arguments
/// 
/// * ˋconfigˋ - search configuration parameters
pub fn run(config: &config::Config) -> Result<(), &'static str> {
  let content = fs::read_to_string(&config.file_path);
  let text = match content {
    Ok(text) => text,
    Err(error) => panic!("Could not load file content: {:?}", error),
  };

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

  #[test]
  #[should_panic(expected="Could not load file content")]
  fn run_panic() {
    let config = Config {
      search_string: String::from("the"),
      file_path: PathBuf::from("fake_file.fake"),
      case_sensitive: true,
    };

    let _ = run(&config);
  }

}
