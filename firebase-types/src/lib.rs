#![doc = include_str!("../README.md")]
//! ## Feature documentation:
#![doc = document_features::document_features!()]

pub mod config;
pub use config::*;
pub mod errors;
pub use errors::*;

#[test]
fn run_test_script() {
	// run ./test.sh
	println!("Running test.sh");
	let mut output = std::process::Command::new("sh")
		.arg("./test.sh")
		.spawn()
		.expect("Failed to run test.sh");
	output.wait().expect("Failed to wait on test.sh");
	// println!("test.sh output: {:?}", output);
	println!("test.sh finished");
}