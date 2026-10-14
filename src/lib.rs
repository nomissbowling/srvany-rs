#![doc(html_root_url = "https://docs.rs/srvany/0.0.1")]
//! srvany for Rust
//!

pub mod core;

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::core::*;

  /// test core
  #[test]
  fn test_core() -> Result<(), Box<dyn Error>> {
    assert_eq!(true, true);
    Ok(())
  }
}
