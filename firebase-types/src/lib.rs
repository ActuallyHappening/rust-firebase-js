pub mod config;
pub use config::*;

/// Tests doctests in README when running `cargo test`, see:
/// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;