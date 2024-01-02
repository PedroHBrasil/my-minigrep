use std::env;

const N_ARGS_PANIC_STR: &str = "not enough arguments";
const N_ARGS: usize = 3; 

pub struct Args {
    pub search_string: String,
    pub file_path: String,
}

impl Args {
    pub fn get() -> Self {
        let args: Vec<String> = env::args().collect();
        Self::check(&args);
        let search_string = args[1].clone();
        let file_path = args[2].clone();

        Self {
            search_string,
            file_path,
        }
    }

    fn check(args: &Vec<String>) {
        if args.len() < N_ARGS {
            panic!("{:?}", N_ARGS_PANIC_STR)
        }
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

        Args::check(&args);
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn check_args_not_enough_args_panic() {
        let args = Vec::from([
            String::from("arg0"),
            String::from("arg1"),
        ]);

        Args::check(&args);
    }

}