pub mod args;

use std::fs;

pub fn run(args: args::Args) {
  let file_string = fs::read_to_string(args.file_path).expect("Couldn't load file.");
  let results = search(&args.search_string, &file_string);
  println!("{:?}", results);
}

fn search<'a>(search_str: &str, text: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in text.lines() {
    if line.contains(search_str) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn search_ok() {
    let search_str = "the";
    let file_path = "./tests/fixtures/poem.txt";
    let file_string = fs::read_to_string(file_path).expect("Couldn't load file.");
    let expected = Vec::from([
      "Then there's a pair of us - don't tell!",
      "To tell your name the livelong day",
    ]);

    let results = search(search_str, &file_string);

    assert_eq!(results, expected);
  }

  #[test]
  fn search_no_match() {
    let search_str = "there's nothing like this";
    let file_path = "./tests/fixtures/poem.txt";
    let file_string = fs::read_to_string(file_path).expect("Couldn't load file.");
    let expected: Vec<&str> = Vec::from([]);

    let results = search(search_str, &file_string);

    assert_eq!(results, expected);
  }
}