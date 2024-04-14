# Overview

femtogrep is a no-frills tool for finding text strings in files.
It has exactly two redeeming features:

- It's default output is very human-readable
- It's very fast. close to 10x faster than grep for support queries. See _Performance_ below.

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
