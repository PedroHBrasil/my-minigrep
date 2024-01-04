/// This module defines the search configuration struct and it's implementation, along with some unit tests. 

use std::env;

/// Error message for the case of missing process arguments
const N_ARGS_PANIC_STR: &str = "not enough arguments";
/// Minimum number of process arguments needed (one more than we actually need because the first one is always the binary file's name)
const N_ARGS: usize = 3; 

/// Represents the configuration needed to search for a string on a text file.
pub struct Config {
    /// The term to search for on the text file contents
    pub search_string: String,
    /// The path to the text file
    pub file_path: String,
    /// Controls if the search is case sensitive or not
    pub case_sensitive: bool,
}

impl Config {
    /// Returns a configuration based on the process arguments and the existance of the "CASE_SENSITIVE" environment variable
    pub fn get() -> Result<Self, &'static str> {
        let args: Vec<String> = env::args().collect();
        Self::check(&args)?;

        let search_string = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Self {
            search_string,
            file_path,
            case_sensitive,
        })
    }

    /// Checks if the quantity of process arguments is enough to define a configuration.
    fn check(args: &Vec<String>) -> Result<(), &'static str> {
        if args.len() < N_ARGS {
            return Err(N_ARGS_PANIC_STR);
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

        assert!(Config::check(&args).is_ok());
    }

    #[test]
    fn check_args_not_enough_args_panic() {
        let args = Vec::from([
            String::from("arg0"),
            String::from("arg1"),
        ]);

        assert!(Config::check(&args).is_err_and(|e| e.eq_ignore_ascii_case(N_ARGS_PANIC_STR)));
    }

}