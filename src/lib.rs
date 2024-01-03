pub mod config;

use std::fs;

pub fn run(config: config::Config) {
  let file_string = fs::read_to_string(config.file_path).expect("Couldn't load file.");
  let results = search(&config.search_string, &file_string, config.case_sensitive);
  println!("{:?}", results);
}

fn search<'a>(search_str: &str, text: &'a str, case_sensitive: bool) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in text.lines() {
    let check_line = get_check_string(line, case_sensitive);
    if check_line.contains(search_str) {
      results.push(line);
    }
  }

  results
}

fn get_check_string(line: &str, case_sensitive: bool) -> String {
  if !case_sensitive {
    return line.to_lowercase();
  }

  String::from(line)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn search_ok_case_sensitive() {
    let search_str = "the";
    let file_path = "./tests/fixtures/poem.txt";
    let file_string = fs::read_to_string(file_path).expect("Couldn't load file.");
    let expected = Vec::from([
      "Then there's a pair of us - don't tell!",
      "To tell your name the livelong day",
    ]);

    let results = search(search_str, &file_string, true);

    assert_eq!(results, expected);
  }

  #[test]
  fn search_ok_case_insensitive() {
    let search_str = "the";
    let file_path = "./tests/fixtures/poem.txt";
    let file_string = fs::read_to_string(file_path).expect("Couldn't load file.");
    let expected = Vec::from([
      "Then there's a pair of us - don't tell!",
      "They'd banish us, you know.",
      "To tell your name the livelong day",
    ]);

    let results = search(search_str, &file_string, false);

    assert_eq!(results, expected);
  }

  #[test]
  fn search_no_match() {
    let search_str = "there's nothing like this";
    let file_path = "./tests/fixtures/poem.txt";
    let file_string = fs::read_to_string(file_path).expect("Couldn't load file.");
    let expected: Vec<&str> = Vec::from([]);

    let results = search(search_str, &file_string, false);

    assert_eq!(results, expected);
  }
}