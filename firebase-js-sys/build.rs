use std::process::Command;

fn main() {
	// println!("cargo:rerun-if-changed=js");
	println!("cargo:rerun-if-changed=js/bundle.ts");
	println!("cargo:rerun-if-changed=js/node.config.mjs");
	println!("cargo:rerun-if-changed=js/web.config.mjs");
	// println!("cargo:rerun-if-changed=tests");

	// #[cfg(feature = "web-not-node")]
	compile_web();

	// #[cfg(feature = "node-not-web")]
	compile_node();
}

fn execute(command: &mut Command) {
	command.output().expect("failed to execute process");
}

// #[cfg(feature = "web-not-node")]
fn compile_web() {
	let mut command = Command::new("rollup");
	command.args(["-c", "js/web.config.mjs"]);
	execute(&mut command);
}

// #[cfg(feature = "node-not-web")]
fn compile_node() {
	let mut command = Command::new("rollup");
	command.args(["-c", "js/node.config.mjs"]);
	execute(&mut command);
}
