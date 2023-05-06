use js_bind::*;

fn main() {
	// build_script_execute();
	println!("cargo:rerun-if-changed=js-bind.toml");
	println!("cargo:rerun-if-changed=js/bundle.ts");

	// run command rollup -c js/web.config.mjs
	let command = "rollup -c js/web.config.mjs";
	let output = std::process::Command::new("sh")
		.arg("-c")
		.arg(command)
		.output()
		.expect("failed to execute process");
}