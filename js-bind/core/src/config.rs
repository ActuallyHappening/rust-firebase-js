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