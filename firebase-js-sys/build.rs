use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=rollup.config.js");
	println!("cargo:rerun-if-changed=js");

	let mut command = Command::new("rollup");
	command.arg("-c");

	if !cfg!(feature = "web") && !cfg!(feature = "node") {
		println!("cargo:warning=[firebase-js-sys,build.rs] No target specified, defaulting to web; set feature 'web' or 'node' to compile JS for that target.");
	}

	command.output().expect("failed to execute process");
}
