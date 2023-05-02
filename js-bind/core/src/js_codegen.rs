use std::{path::PathBuf, process::Command};

use crate::config::{Config, ConfigLock, FromTOMLCwd, LockTemplate, Template};

/// Is the only function you need to call in your build script to handle everything
/// to do with `js-bind`!
pub fn build_script_execute() {
	println!("cargo:rerun-if-changed=js-bind.toml");
	println!("cargo:rerun-if-changed=js-bind.lock");

	let cwd = std::env::current_dir().expect("to work");
	let config = Config::from_cwd().expect("Cannot parse config");

	let lock = ConfigLock::from_cwd().expect("Cannot parse config lock");

	let mut bundle = JsCodegenFile::new();
	lock.templates.into_iter().for_each(|func| {
		bundle.add_template(&config, func);
	});
	let path = cwd.join(&config.codegen.output);
	// eprintln!("Writing bundle: {:?}", &bundle);
	bundle.write_at_file(&path);

	config.bundles.into_iter().for_each(|module| {
		let cmd_handle =
			RollupHandle::new(&cwd, config.codegen.output.clone(), module.to_build_command);
		cmd_handle.compile();
	});
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

impl Template {
	pub fn expand(&self, lock_template: &LockTemplate) -> String {
		let mut template = self.template.clone();
		template = template.replace("{{#name}}", lock_template.var_name.as_str());
		template = template.replace("{{#mod}}", lock_template.var_module.as_str());
		template
	}
}

// For linking
impl JsCodegenFile {
	/// Adds an expanded template to the file
	pub fn add_template(&mut self, config: &Config, lock_template: LockTemplate) {
		let err_msg = format!(
			"Function uses mode {} that is not defined in config",
			lock_template.template_name_ref
		);

		let template = config
			.codegen
			.templates
			.clone() // TODO: Maybe not for performance?
			.into_iter()
			.find(|t| t.name == lock_template.template_name_ref)
			.expect(&err_msg);

		let expanded = template.expand(&lock_template);
		self.lines.push(expanded);
	}

	pub fn write_at_file(&self, file_path: &PathBuf) {
		let data = self.lines.join("\n");
		std::fs::write(file_path, &data)
			.expect(format!("Failed to write file at path: {:?}", file_path).as_str());
	}
}

struct RollupHandle {
	pub command: String,
	pub expected_output_file: PathBuf,
}

impl RollupHandle {
	pub fn new(cwd: &PathBuf, expected_output_file: String, command: String) -> Self {
		let expected_output_file = cwd.join(expected_output_file);
		Self {
			expected_output_file,
			command,
		}
	}

	pub fn compile(&self) {
		println!("Running to-build command: {:?}", &self.command);
		let output = Command::new(&self.command)
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

	fn check_output(&self) {
		let output_path = &self.expected_output_file;
		if !output_path.exists() {
			panic!(
				"Expected output file does not exist at path: {:?}",
				&output_path
			);
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
