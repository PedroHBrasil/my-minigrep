use std::error::Error;

const N_ARGS: usize = 3; 

pub fn get_args() -> Result<Vec<String>, Box<dyn Error>> {
    unimplemented!();
}

fn check_args() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

#[cfg(test)]
mod test {
    #[test]
    fn get_args_ok() {
        unimplemented!()
    }

    #[test]
    fn get_args_no_args_panic() {
        unimplemented!()
    }

    #[test]
    fn get_args_file_not_found_panic() {
        unimplemented!()
    }

    #[test]
    fn get_args_missing_search_string_panic() {
        unimplemented!()
    }

}