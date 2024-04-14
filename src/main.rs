use clap::Parser;
use femtogrep::*;
use std::{env, process};

#[derive(Parser, Debug)]
#[command(author = "Matt Gadda", version, about = "No frills portable string searching", long_about = None)]
struct FemtoGrepArgs {
  /// Search recursively
  #[arg(short, long, default_value_t = false)]
  recursive: bool,

  /// The expression to search for
  expression: String,

  /// Paths to search within
  file_paths: Vec<String>,
}

fn main() {
  let args = FemtoGrepArgs::parse();
  let config = Config {
    query: &args.expression,
    file_paths: &args.file_paths,
    recursive: args.recursive,
  };

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
