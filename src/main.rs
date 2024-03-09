use clap::Parser;
use minigrep::*;
use std::{env, process};

#[derive(Parser, Debug)]
#[command(author = "Matt Gadda", version, about = "No frills portable string searching", long_about = None)]
struct MinigrepArgs {
  /// Search recursively
  #[arg(short, long, default_value_t = false)]
  recursive: bool,

  /// The expression to search for
  expression: String,

  /// Paths to search within
  file_paths: Vec<String>,
}

fn main() {
  let args = MinigrepArgs::parse();
  let config = Config {
    query: &args.expression,
    file_paths: &args.file_paths,
    recursive: args.recursive,
  };

  // let args: Vec<String> = env::args().collect();
  // let config = Config::build(&args).unwrap_or_else(|err| {
  //   eprintln!("Problem parsing arguments: {}", err);
  //   process::exit(1);
  // });
  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
