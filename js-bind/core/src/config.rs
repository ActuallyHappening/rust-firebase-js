use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
	/// ```rs
	/// let specific_mode: Mode = modes['name']
	/// ```
	// modes: HashMap<String, Mode>,
	/// [[files.bundle]]
	/// on-feature="foo"
	/// path="bar"
	///
	/// expands to
	///
	/// files: {
	/// 	bundle: [{
	/// 		on-feature: "foo",
	/// 		path: "bar",
	/// 	}]
	/// }
	/// ```rs
	///	let file_ref: File = files['bundle']
	// files: HashMap<String, File>,
	build: Build,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Build {
	#[serde(rename = "js-codegen-file")]
	js_codegen_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mode {
	#[serde(rename = "file-ref")]
	file_ref: String,

	#[serde(rename = "js-export-name")]
	js_export_name: String,

	#[serde(rename = "js-export-type")]
	js_casing: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct File {}

impl Config {
	pub fn from_toml_config_str(string: &str) -> anyhow::Result<Config> {
		toml::from_str(string)
		.context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}
	pub fn from_config_dir(path: &str) -> anyhow::Result<Config> {
		let config_path = format!("{}/js-bind.toml", path);
		let err_msg = format!(
			"Couldn't read file 'js-bind.toml' at path {:?}",
			&config_path
		);
		let config_str = std::fs::read_to_string(&config_path).context(err_msg)?;
		Ok(Config::from_toml_config_str(&config_str)?)
	}
}
