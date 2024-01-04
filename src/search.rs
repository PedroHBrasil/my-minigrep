/// This module defines the search function and any a support function for it, along with some unit tests. 

/// Returns the lines of the provided text that contain the given string
/// 
/// # Arguments
/// 
/// * ˋsearch_strˋ - The term to search for on ˋtextˋ
/// * ˋtextˋ - The text to get the lines from
/// * ˋcase_sensitiveˋ - Controls if the matching is case sensitive or not
/// 
/// # Examples
/// 
/// ˋˋˋ
///   const TEST_SEARCH_STR: &str = "the";
///   const TEST_TEXT: &str = "
/// Let's see if this works.
/// This line should not be included.
/// But there's at least one that should.
/// Lines with 'the' and/or 'The' should be included.
/// The set up of an env var 'CASE_SENSITIVE' determines is this line is present or not, at least for TEST_SEARCH_STR as input.
/// ";
/// let expected = Vec::from([
///   "But there's at least one that should.",
///   "Lines with 'the' and/or 'The' should be included.",
/// ]);
/// 
/// let results = search(TEST_SEARCH_STR, TEST_TEXT, true);
/// 
/// assert_eq!(results, expected);
/// ˋˋˋ
/// 
pub fn search<'a>(search_str: &str, text: &'a str, case_sensitive: bool) -> Vec<&'a str> {
  let check_search_str = get_check_string(search_str, case_sensitive);

  let mut results = Vec::new();
  for line in text.lines() {
    let check_line = get_check_string(line, case_sensitive);
    if check_line.contains(&check_search_str) {
      results.push(line);
    }
  }

  results
}

/// Returns the strings that should be used on the search process.
/// 
/// # Arguments
/// 
/// * ˋstringˋ - A string used on the search process
/// * ˋcase_sensitiveˋ - If true, the string is equal to the original line. Else, the string is the lower case version of the original.
/// 
/// # Examples
/// 
/// ˋˋˋ
/// let string = "TeSt";
/// let expected = "test";
/// 
/// let result = get_check_string(string, false);
/// 
/// assert_eq!(result, expected);
/// ˋˋˋ
fn get_check_string(string: &str, case_sensitive: bool) -> String {
  if !case_sensitive {
    return string.to_lowercase();
  }

  String::from(string)
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

  #[test]
  fn get_check_string_case_sensitive() {
    let string = "TeSt";

    let result = get_check_string(string, true);

    assert_eq!(result, string);
  }

  #[test]
  fn get_check_string_case_insensitive() {
    let string = "TeSt";
    let expected = "test";

    let result = get_check_string(string, false);

    assert_eq!(result, expected);
  }
}