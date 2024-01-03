use my_minigrep::{config::Config, run};

fn main() {
  let config = Config::get();
  run(config);
}
