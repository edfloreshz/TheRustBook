use std::error::Error;
use std::fs;

pub struct Config<'a> {
  pub query: &'a str,
  pub filename: &'a str,
}

impl Config<'_> {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    Ok(Config {
      query: &args[1],
      filename: &args[2],
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let _ = fs::read_to_string(config.filename)?;
  Ok(())
}
