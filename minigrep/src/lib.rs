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
  for line in search_case_sensitive(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "productive";
    assert_eq!(vec!["productive"], search_case_sensitive(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "The Rust Programming Language
Trust me";
    assert_eq!(
      vec!["The Rust Programming Language", "Trust me"],
      search_case_insensitive(query, contents)
    );
  }
}
