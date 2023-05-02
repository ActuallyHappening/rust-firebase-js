use std::path::PathBuf;

use crate::config::{Config, ConfigLock, FromTOMLCwd, Template, LockTemplate};

/// Is the only function you need to call in your build script to handle everything
/// to do with `js-bind`!
pub fn build_script_execute() {
	println!("cargo:rerun-if-changed=js-bind.toml");
	println!("cargo:rerun-if-changed=js-bind.lock");

	let cwd = std::env::current_dir().expect("to work");
	let config = Config::from_cwd().expect("Cannot parse config");

	prepare(&config, &cwd);
	// clear_lockfile(&cwd);

	let lock = ConfigLock::from_cwd().expect("Cannot parse config lock");

	let mut bundle = JsCodegenFile::new();
	lock.templates.into_iter().for_each(|func| {
		// panic!("YES");
		bundle.add_template(&config, func);
	});
	let path = cwd.join(config.build.codegen.generic_bundle);
	// eprintln!("Writing bundle: {:?}", &bundle);
	bundle.write_at_file(&path);

	let node_handle = RollupHandle::new(
		&cwd,
		config.build.target.node.rollup_config,
		config.build.target.node.bundle_name,
	);
	let web_handle = RollupHandle::new(
		&cwd,
		config.build.target.web.rollup_config,
		config.build.target.web.bundle_name,
	);
	node_handle.compile();
	web_handle.compile();
}

/// Represents a file to be outputted by the codegen engine.
/// This could be to link to a nodejs module, or to a web module.
/// Or, it could be a file that is for testing from doctests.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JsCodegenFile {
	lines: Vec<String>,
}

impl JsCodegenFile {
	pub fn new() -> Self {
		Self { lines: Vec::new() }
	}
}

// For linking
impl JsCodegenFile {
	/// Adds an expanded template to the file
	pub fn add_template(&mut self, config: &Config, func: LockTemplate) {
		let tepmlate = config.codegen.templates.get(&func.mode_name).expect(
			format!(
				"Function uses mode {} that is not defined in config",
				func.mode_name
			)
			.as_str(),
		);

		
	}

	pub fn write_at_file(&self, file_path: &PathBuf) {
		let data = self.lines.join("\n");
		std::fs::write(file_path, &data)
			.expect(format!("Failed to write file at path: {:?}", file_path).as_str());
	}
}

struct RollupHandle {
	pub config_file: PathBuf,
	pub expected_output_file: PathBuf,
}

impl RollupHandle {
	pub fn new(cwd: &PathBuf, config_file: String, expected_output_file: String) -> Self {
		let config_file = cwd.join(config_file);
		let expected_output_file = cwd.join(expected_output_file);
		println!("Config file: {:?},\n output file: {:?},\n cwd: {:?}", &config_file, &expected_output_file, &cwd);
		Self {
			config_file,
			expected_output_file,
		}
	}

	pub fn compile(&self) {
		println!("Compiling with rollup config: {:?}", &self.config_file);
		let output = std::process::Command::new("rollup")
			.arg("-c")
			.arg(&self.config_file)
			.output()
			.expect("Rollup didn't exit properly");
		if !output.status.success() {
			panic!(
				"Rollup failed to compile. Output: {}",
				String::from_utf8_lossy(&output.stderr)
			);
		}

		self.check_output();
	}

	fn check_output(&self,) {
		let output_path = &self.expected_output_file;
		if !output_path.exists() {
			panic!("Expected output file does not exist at path: {:?}", &output_path);
		} else {
			println!("Output file exists at {:?}", &output_path);
		}
	}
}

pub fn clear_lockfile(dir: &PathBuf) {
	let file = dir.join("js-bind.lock");
	// remove file
	if file.exists() {
		std::fs::remove_file(file).expect("Failed to remove lockfile");
	} else {
		println!("No lockfile to remove, skipping");
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
			eprintln!(
				"The tsconfig.json file '{:?}' does not exist. This is not necessary, but recommended.",
				&tsconfig
			);
		}
	} else {
		println!("Not using typescript")
	}
}
