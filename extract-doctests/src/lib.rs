#![doc = include_str!("../README.md")]

pub use extract_doctests_proc::extract_doctests;

#[test]
fn ui_tests() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/*.rs");
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