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

#[test]
fn run_examples() {
	// run cargo run --example foo
	// for every file *.rs in examples
	let mut examples = std::fs::read_dir("examples").unwrap()
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, std::io::Error>>().unwrap();

	examples.sort();

	for example in examples {
		if example.extension().unwrap() == "rs" {
			println!("Running example: {:?}", example);
			let mut output = std::process::Command::new("cargo")
				.arg("run")
				.arg("--example")
				.arg(example.file_stem().unwrap())
				.spawn()
				.expect("Failed to run example");
			output.wait().expect("Failed to wait on example");
			// println!("example output: {:?}", output);
			println!("example finished");
		}
	}
}