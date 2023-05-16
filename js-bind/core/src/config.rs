use std::path::PathBuf;

use serde::{Deserialize, Serialize};

///```rust
/// use js_bind_core::config::Config;
///
/// let toml_str = &std::fs::read_to_string("../examples/testing-configs/js-bind.toml").expect("Couldn't read file");
///
/// let config = toml::from_str::<Config>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Config {
	pub bundles: Option<Vec<Bundle>>,
	pub codegen: Option<CodeGen>,

	full_path: Option<PathBuf>,
}

/// ```rust
/// use js_bind_core::config::Bundle;
///
/// let toml_str = r#"
/// if = "link-node"
/// then = "something/path.js"
/// to-build = "rollup -args"
/// "#;
///
/// let config = toml::from_str::<Bundle>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Bundle {
	/// Feature name
	#[serde(rename = "if")]
	pub if_feature: String,
	/// Path to js file
	#[serde(rename = "then")]
	pub then_js_path: String,
	/// Command to build the file (e.g. rollup -c web.config.mjs)
	#[serde(rename = "to-build")]
	pub to_build_command: String,
}

/// Represents the JS bundle codegen part of the config
///
/// ```rust
/// use js_bind_core::config::CodeGen;
///
/// let toml_str = r#"
/// output = "js-bind.lock"
/// template = "NA"
/// "#;
///
/// let config = toml::from_str::<CodeGen>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CodeGen {
	/// Relative file name
	pub output: String,
	/// Contains variables #name and #mod
	pub template: String,
}

// Impls

impl Config {
	/// Gets the config from the current working directory,
	/// using the relative path provided.
	///
	/// ## Example
	/// ```rust,no_run
	/// use js_bind_core::config::Config;
	///
	/// # // Change dir to directory for doctests
	/// # std::env::set_current_dir("../examples/testing-configs").expect("to change dir");
	/// // create file
	/// // std::fs::write("js-bind.toml", r#"
	/// // [[bundles]]
	/// // if = "link-node"
	/// // then = "something/path.js"
	/// // to-build = "rollup -args"
	/// //
	/// // e.t.c.
	/// // "#).expect("to write file");
	///
	/// // Parses the file `js-bind.toml` in the current working directory
	/// // as a config file
	/// let config = Config::from_package_root("js-bind.toml");
	/// config.expect("to parse");
	/// ```
	pub fn from_package_root(relative_path: &str) -> Result<Self, toml::de::Error> {
		// let relative_dir = std::env::current_dir().expect("Cannot locate cwd");
		let relative_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir not provided through env var CARGO_MANIFEST_DIR").into();
		// let relative_dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();

		eprintln!("cwd: {}", relative_dir.to_str().unwrap());

		let full_path = relative_dir.join(relative_path);

		let string = std::fs::read_to_string(full_path.clone()).expect(
			format!(
				"Couldn't read file (relative: {:?}, relative_dir(e.g. package root): {:?}): {:?}",
				relative_path, relative_dir, full_path
			)
			.as_str(),
		);

		let mut config = toml::from_str::<Config>(string.as_str())?;
		config.full_path = Some(full_path);
		Ok(config)
	}
}
