/// This module defines the search configuration struct and it's implementation, along with some unit tests.
use std::{ env, path };

/// Error messages
const N_ARGS_PANIC_STR: &str = "not enough arguments";
const FILE_DOESNT_EXIST_STR: &str = "file does not exist";
const PATH_IS_DIR_STR: &str = "path is a directory";
/// Minimum number of process arguments needed (one more than we actually need because the first one is always the binary file's name)
const N_ARGS: usize = 3;

/// Represents the configuration needed to search for a string on a text file.
pub struct Config {
  /// The term to search for on the text file contents
  pub search_string: String,
  /// The path to the text file
  pub file_path: path::PathBuf,
  /// Controls if the search is case sensitive or not
  pub case_sensitive: bool,
}

impl Config {
  /// Returns a configuration based on the process arguments and the existance of the "CASE_SENSITIVE" environment variable
  pub fn get() -> Result<Self, &'static str> {
    let args: Vec<String> = env::args().collect();
    Self::check_args(&args)?;

    let search_string = args[1].clone();
    let file_path = path::PathBuf::from(args[2].clone());
    Self::check_file_path(&file_path)?;

    let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

    Ok(Self {
      search_string,
      file_path,
      case_sensitive,
    })
  }

  /// Checks if the quantity of process arguments is enough to define a configuration.
  fn check_args(args: &Vec<String>) -> Result<(), &'static str> {
    if args.len() < N_ARGS {
      return Err(N_ARGS_PANIC_STR);
    }

    Ok(())
  }

  fn check_file_path(file_path: &path::PathBuf) -> Result<(), &'static str> {
    if !file_path.exists() {
      return Err(FILE_DOESNT_EXIST_STR);
    }
    if file_path.is_dir() {
      return Err(PATH_IS_DIR_STR);
    }

    Ok(())
  } 
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_args_ok() {
    let args = Vec::from([
      String::from("arg0"),
      String::from("arg1"),
      String::from("arg2"),
    ]);

    let result = Config::check_args(&args);

    assert!(result.is_ok());
  }

  #[test]
  fn check_args_not_enough_args_error() {
    let args = Vec::from([String::from("arg0"), String::from("arg1")]);

    let result = Config::check_args(&args);

    assert!(result.is_err_and(|e| e.eq_ignore_ascii_case(N_ARGS_PANIC_STR)));
  }

  #[test]
  fn check_file_path_ok() {
    let file_path = path::PathBuf::from("tests/fixtures/poem.txt");

    let result = Config::check_file_path(&file_path);

    assert!(result.is_ok());
  }

  #[test]
  fn check_file_path_file_doesnt_exist() {
    let file_path = path::PathBuf::from("tests/fixtures/doesnt_exist.crazy");

    let result = Config::check_file_path(&file_path);

    assert!(result.is_err_and(|e| e.eq_ignore_ascii_case(FILE_DOESNT_EXIST_STR)));
  }

  #[test]
  fn check_file_path_is_dir() {
    let file_path = path::PathBuf::from("tests");

    let result = Config::check_file_path(&file_path);

    assert!(result.is_err_and(|e| e.eq_ignore_ascii_case(PATH_IS_DIR_STR)));
  }
}
