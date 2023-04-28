use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=web.config.js");
	println!("cargo:rerun-if-changed=node.config.js");
	println!("cargo:rerun-if-changed=js");

	// cfg_if! {
	// 	if #[cfg(feature = "web")] {
	// 		compile_web();
	// 	} else if #[cfg(feature = "node")] {
	// 		compile_node();
	// 	} else {
	// 		println!("cargo:warning=[firebase-js-sys,build.rs] No target specified, defaulting to web; set feature 'web' or 'node' to compile JS for that target.");
	// 	}
	// }

	#[cfg(feature = "web")]
	compile_web();

	#[cfg(not(feature = "node"))]
	compile_node();
}

fn execute(command: &mut Command) {
	command.output().expect("failed to execute process");
}

fn compile_web() {
	let mut command = Command::new("rollup");
	command.args(["-c", "js/web.config.js"]);
	execute(&mut command);
}

fn compile_node() {
	let mut command = Command::new("rollup");
	command.args(["-c", "js/node.config.js"]);
	execute(&mut command);
}
