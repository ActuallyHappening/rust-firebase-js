use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
	pub build: Build,

	pub modes: HashMap<String, Mode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Build {
	pub codegen: CodeGenBuild,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CodeGenBuild {
	#[serde(rename = "output-file")]
	pub output_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mode {
	#[serde(rename = "mod")]
	pub mod_name: String,

	#[serde(rename = "type")]
	pub item_type: String,

	// #[serde(rename = "js-export-type")]
	// js_casing: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct File {}

impl Config {
	pub fn from_toml_config_str(string: &str) -> anyhow::Result<Config> {
		toml::from_str(string)
		.context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}
	pub fn from_config_dir(mut path: PathBuf) -> anyhow::Result<Config> {
		path.push("js-bind");
		path.set_extension("toml");
		let err_msg = format!(
			"Couldn't read file 'js-bind.toml' at path {:?}",
			&path
		);
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Config::from_toml_config_str(&config_str)?)
	}
}

pub struct ConfigLock {

}