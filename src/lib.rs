use std::env;

const N_ARGS_PANIC_STR: &str = "not enough arguments";
const N_ARGS: usize = 3; 

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    check_args(&args);

    args
}

fn check_args(args: &Vec<String>) {
    if args.len() < N_ARGS {
        panic!("{:?}", N_ARGS_PANIC_STR)
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

        check_args(&args);
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn check_args_not_enough_args_panic() {
        let args = Vec::from([
            String::from("arg0"),
            String::from("arg1"),
        ]);

        check_args(&args);
    }

}