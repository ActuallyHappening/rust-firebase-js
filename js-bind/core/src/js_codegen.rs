use std::path::PathBuf;

use crate::config::{Config, ConfigLock, Function, Target};

/// Represents a file to be outputted by the codegen
struct JsCodegenFile {
	pub functions: Vec<Function>,
}

/// Is the only function you need to call in your build script to handle everything
/// to do with `js-bind`!
pub fn build_script_execute() {
	let cwd = std::env::current_dir().expect("to work");
	let config = Config::from_config_dir(&cwd).expect("Cannot parse config");
	prepare(&config, &cwd);
	
	let lock = ConfigLock::from_config_dir(&cwd).expect("Cannot parse config lock");
}

struct RollupHandle {

}

impl RollupHandle {
	pub fn prepare() {

	}
}

/// Checks the `npm_driver` is installed,
/// Checks that `rollup` is installed,
/// Checks that the `rollup` config file exists,
/// Checks that the `tsconfig.json` file exists (if ts support is specified),
/// 
/// panics if any of the above are not true
pub fn prepare(config: &Config, cwd: &PathBuf) {
	let npm_driver = &config.build.codegen.npm_driver;
	// Check that the npm driver is installed
	let npm_driver_installed = std::process::Command::new(&npm_driver)
		.arg("--version")
		.output()
		.expect("to work")
		.status
		.success();
	if !npm_driver_installed {
		panic!("The npm driver '{}' is not installed", &npm_driver);
	} else {
		println!("Using npm driver '{}'", &npm_driver)
	}

	// Check that rollup is installed
	let rollup_installed = std::process::Command::new("rollup")
		.arg("--version")
		.output()
		.expect("to work")
		.status
		.success();
	if !rollup_installed {
		panic!("Rollup is not installed");
	} else {
		println!("Using rollup");
	}

	// Check that the rollup config file exists
	fn check_file(path: String) {
		let path: PathBuf = path.into();
		if !path.exists() {
			panic!("The rollup config file '{:?}' does not exist at ", &path);
		} else {
			println!("Using rollup config file '{:?}'", &path);
		}
	}
	check_file(config.build.target.node.rollup_config.clone());
	check_file(config.build.target.web.rollup_config.clone());

	// Check that the tsconfig.json file exists
	if config.build.codegen.ts {
		let tsconfig = cwd.join("tsconfig.json");
		if !tsconfig.exists() {
			eprintln!("The tsconfig.json file '{:?}' does not exist. This is not necessary, but recommended.", &tsconfig);
		}
	}

}