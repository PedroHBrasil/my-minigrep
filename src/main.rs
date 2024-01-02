use my_minigrep::{args::Args, run};

fn main() {
  let args = Args::get();
  run(args);
}
