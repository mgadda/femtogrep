# Overview

femtogrep is a few-frills tool for finding text strings in files.
it has two redeeming features:

- it's default output is very human-readable
- it's very fast. like, 10x faster than grep. See _Performance_ below.

# Installation

## Build from source

cargo install femtogrep

# Usage

```
femtogrep --help
No frills portable string searching

Usage: femtogrep [OPTIONS] <EXPRESSION> [FILE_PATHS]...

Arguments:
  <EXPRESSION>     The expression to search for
  [FILE_PATHS]...  Paths to search within

Options:
  -r, --recursive  Search recursively
  -h, --help       Print help
  -V, --version    Print version
```

# Performance

TODO: add benchmark results
