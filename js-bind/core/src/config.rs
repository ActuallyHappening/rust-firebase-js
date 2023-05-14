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
	#[serde(rename = "doctestgen")]
	pub doc_test_gen: Option<DocTestGen>,

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

/// Represents the doc-test codegen part of the config
/// 
/// ```rust
/// use js_bind_core::config::DocTestGen;
/// 
/// let toml_str = r#"
/// template = "NA"
/// web-feature-name = "yay"
/// "#;
/// 
/// let config = toml::from_str::<DocTestGen>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DocTestGen {
	/// Contains variables #web_feature_name, #test_name and #code
	pub template: String,
}

// Impls

impl Config {
	/// Gets the config from the current working directory,
	/// using the relative path provided.
	/// 
	/// ## Example
	/// ```rust
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
	/// let config = Config::from_cwd("js-bind.toml");
	/// config.expect("to parse");
	/// ```
	pub fn from_cwd(relative_path: &str) -> Result<Self, toml::de::Error> {
		let cwd = std::env::current_dir().expect("Cannot locate cwd");
		let full_path = cwd.join(relative_path);

		let string = std::fs::read_to_string(full_path.clone()).expect(format!("Couldn't read file (relative: {:?}): {:?}", relative_path, full_path).as_str());

		let mut config = toml::from_str::<Config>(string.as_str())?;
		config.full_path = Some(full_path);
		Ok(config)
	}
}

