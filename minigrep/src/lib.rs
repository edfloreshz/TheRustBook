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
  let contents = fs::read_to_string(config.filename)?;
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "productive";
    assert_eq!(vec!["productive"], search(query, contents));
  }
}
