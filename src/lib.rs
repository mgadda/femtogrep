use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::result::Result;
use std::str::RMatchIndices;
use termion::color;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // let mut paths_to_process: Vec<&str> = config.file_paths.iter().map(|f| f.as_str()).collect();
  // let mut paths_to_process: Vec<String> = Vec::from(config.file_paths);

  let mut paths_to_process: VecDeque<String> = VecDeque::new();
  paths_to_process.extend(config.file_paths.iter().map(String::clone));

  while let Some(file_path) = paths_to_process.pop_front() {
    // extracting actual paths from read_dir is extraordinarily painful
    // on account of it returning a Result<ReadDir<Result<DirEntry>>>
    let path = Path::new(&file_path);
    if path.is_dir() && config.recursive {
      if let Ok(result) = fs::read_dir(&file_path) {
        let new_paths: Vec<String> = result
          .filter_map(|dir_entry_result| -> Option<String> {
            dir_entry_result.map_or(None, |dir_entry: DirEntry| -> Option<String> {
              dir_entry.path().to_str().map(|f| f.to_string())
            })
          })
          .collect();

        // can this be replaced with `extend`?
        for path in new_paths {
          paths_to_process.push_back(path);
        }
      }

      // paths_to_process.extend()
    } else if path.is_file() {
      let _ = run_one(config.query, &file_path);
    }
  }

  Ok(())
}

pub fn run_one(query: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(file_path)?;

  let red = color::Fg(color::Red);
  let reset = color::Fg(color::Reset);

  let mut found_matches = false;

  for line in search(query, &contents) {
    let mut formatted_line = String::from(line.0);
    for (start, str) in line.1 {
      // insert reset string before inserting color string
      // since insertions invalidate indices
      formatted_line.insert_str(start + str.len(), &format!("{}", reset));
      formatted_line.insert_str(start, &format!("{}", red));
    }

    if !found_matches {
      // only print out the filename if we actually have found at least one match
      println!(
        "\n{}{file_path}{}:",
        color::Fg(color::Blue),
        color::Fg(color::Reset)
      );
      found_matches = true;
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
  pub recursive: bool,
}

impl<'a> Config<'a> {
  pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = &args[1]; //.clone();
    let file_paths = &args[2..]; //.clone();

    Ok(Config {
      query,
      file_paths,
      recursive: true,
    })
  }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<(&'a str, RMatchIndices<'a, &'a str>)> {
  // 'a lifetime var connects the lifetime of contents to the return value
  // since the return value is a list of str references from contents
  contents
    .split("\n")
    .filter_map(|line| {
      if line.contains(query) {
        Some((line, line.rmatch_indices(query)))
      } else {
        None
      }
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn one_result() {
    let query = "Pick";
    let contents = "\
Rust
safe, fast, productive.
Pick three.
    ";

    assert_eq!("Pick", search(query, contents)[0].0);
  }
}
