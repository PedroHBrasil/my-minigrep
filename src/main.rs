use my_minigrep::{config::Config, run};

fn main() -> Result<(), &'static str> {
  let config = Config::get()?;
  run(&config)?;

  Ok(())
}
