//! srvany.rs
//!
//! - cargo build --bins --release
//! - cargo build --bin srvany --release
//!

use srvany::core::*;

use std::path::PathBuf;
use bpaf::{self, Bpaf, Parser};

use std::{result::Result, error::Error};

/// Opts
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Opts {
  /// nocapture (only for test)
  #[bpaf(long)]
  nocapture: bool,
  /// show_output (only for test)
  #[bpaf(long)]
  show_output: bool,
  /// verbose
  #[bpaf(external(verbose))]
  verbose: usize,
  /// activate debug
  #[bpaf(short, long)]
  debug: bool
}

/// external verbose
pub fn verbose() -> impl Parser<usize> {
  bpaf::short('v').long("verbose").help("verbose -v or -vv")
    .req_flag(()).many().map(|xs| xs.len())
    .guard(|&x| x <= 2, "verbose <= 2")
}

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  let op = opts().run();
  if op.debug {
    println!("{:?}", op);
    println!("--nocapture: {}", op.nocapture); // (only for test)
    println!("--show-output: {}", op.show_output); // (only for test)
    if op.verbose < 1 { return Ok(()); }
  }
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use srvany::core::Srv; // lib
  use crate::*; // main

  /// test srvany
  #[test]
  fn test_srvany() -> Result<(), Box<dyn Error>> {
    let op = opts().run();
    assert_eq!(op.verbose, 0);
    Ok(())
  }
}
