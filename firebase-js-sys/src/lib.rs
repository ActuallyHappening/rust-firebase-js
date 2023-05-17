#![doc = include_str!("../README.md")]
//! ## Features:
#![doc = document_features::document_features!()]

// compile error is niether feature is enabled
#[cfg(not(any(feature = "web-not-node", feature = "node-not-web")))]
compile_error!("You must enable either the `web-not-node` or `node-not-web` feature to use this crate");
// compile error if both features are enabled
// #[cfg(all(feature = "web-not-node", feature = "node-not-web"))]
// compile_error!("You must enable either the `web-not-node` or `node-not-web` feature to use this crate, not both");

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

pub mod app;
pub use app::*;

#[cfg(test)]
pub mod testing;