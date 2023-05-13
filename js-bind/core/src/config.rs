use std::path::PathBuf;

use serde::{Deserialize, Serialize};

///```rust
/// use js_bind_core::config::Config;
///
/// let toml_str = &std::fs::read_to_string("../../js-bind-tests/js-bind.toml").expect("Couldn't read file");
///
/// let config = toml::from_str::<Config>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Config {
	pub bundles: Vec<Bundle>,
	pub codegen: CodeGen,

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

/// Represents the codegen side of the config, e.g. documentation + test generation
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
	pub output: String,
	pub template: String,
}

// Impls

const CONFIG_FILE_NAME: &str = "js-bind.toml";

impl Config {
	/// Gets the config from the current working directory
	/// 
	/// ```rust
	/// use js_bind_core::config::Config;
	/// 
	/// # // Change dir to directory for doctests
	/// # std::env::set_current_dir("../examples/doctest").expect("to change dir");
	/// 
	/// // create file
	/// // std::fs::write("js-bind.toml", r#"
	/// // [[bundles]]
	/// // if = "link-node"
	/// // then = "something/path.js"
	/// // to-build = "rollup -args"
	/// // "#).expect("to write file");
	/// 
	/// let config = Config::from_cwd();
	/// config.expect("to parse");
	/// ```
	pub fn from_cwd() -> Result<Self, toml::de::Error> {
		let cwd = std::env::current_dir().expect("Cannot locate cwd");
		let full_path = cwd.join(CONFIG_FILE_NAME);
		let string = std::fs::read_to_string(full_path.clone()).expect("Couldn't read file");
		let mut config = toml::from_str::<Config>(string.as_str())?;
		config.full_path = Some(full_path);
		Ok(config)
	}
}
