pub fn search<'a>(search_str: &str, text: &'a str, case_sensitive: bool) -> Vec<&'a str> {
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

  const TEST_SEARCH_STR: &str = "the";
  const TEST_TEXT: &str = "
Let's see if this works.
This line should not be included.
But there's at least one that should.
Lines with 'the' and/or 'The' should be included.
The set up of an env var 'CASE_SENSITIVE' determines is this line is present or not, at least for TEST_SEARCH_STR as input.
";

  #[test]
  fn search_ok_case_sensitive() {
    let expected = Vec::from([
      "But there's at least one that should.",
      "Lines with 'the' and/or 'The' should be included.",
    ]);

    let results = search(TEST_SEARCH_STR, TEST_TEXT, true);

    assert_eq!(results, expected);
  }

  #[test]
  fn search_ok_case_insensitive() {
    let expected = Vec::from([
      "But there's at least one that should.",
      "Lines with 'the' and/or 'The' should be included.",
      "The set up of an env var 'CASE_SENSITIVE' determines is this line is present or not, at least for TEST_SEARCH_STR as input."
    ]);

    let results = search(TEST_SEARCH_STR, TEST_TEXT, false);

    assert_eq!(results, expected);
  }

  #[test]
  fn search_no_match() {
    let search_str = "there's nothing like this";
    let expected: Vec<&str> = Vec::from([]);

    let results = search(search_str, TEST_SEARCH_STR, false);

    assert_eq!(results, expected);
  }
}