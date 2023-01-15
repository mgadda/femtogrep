use std::error::Error;
use std::fs;
use std::result::Result;
use std::str::RMatchIndices;
use termion::color;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  config
    .file_paths
    .iter()
    .map(|file_path| run_one(config.query, file_path))
    .collect()
}

pub fn run_one(query: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(file_path)?;

  let red = color::Fg(color::Red);
  let reset = color::Fg(color::Reset);

  println!(
    "{}{file_path}{}:",
    color::Fg(color::Blue),
    color::Fg(color::Reset)
  );

  for line in search(query, &contents) {
    let mut formatted_line = String::from(line.0);
    for (start, str) in line.1 {
      // insert reset string before inserting color string
      // since insertions invalidate indices
      formatted_line.insert_str(start + str.len(), &format!("{}", reset));
      formatted_line.insert_str(start, &format!("{}", red));
    }

    println!("  {formatted_line}");
  }

  Ok(())
}

pub struct Config<'a> {
  // n.b. if we clone args then when we can make these owned Strings instead
  // which might be altogether easier to read, especially considering that in
  // practice, the string references that will be held here come from values
  // that live for the duration of the program.
  pub query: &'a str,
  pub file_paths: &'a [String],
}

impl<'a> Config<'a> {
  pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = &args[1]; //.clone();
    let file_paths = &args[2..]; //.clone();

    Ok(Config { query, file_paths })
  }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(&'a str, RMatchIndices<'a, &'a str>)> {
  // 'a lifetime var connects the lifetime of contents to the return value
  // since the return value is a list of str references from contents
  let mut matches: Vec<(&str, RMatchIndices<&str>)> = Vec::new();

  for line in contents.split("\n") {
    if line.contains(query) {
      matches.push((line, line.rmatch_indices(query)));
    }
  }

  matches
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust
safe, fast, productive.
Pick three.
    ";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }
}
