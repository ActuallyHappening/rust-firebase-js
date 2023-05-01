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
	pub fn from_toml_config_str(string: &str) -> anyhow::Result<Self> {
		toml::from_str(string).context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}
	pub fn from_config_dir(path: &mut PathBuf) -> anyhow::Result<Self> {
		path.push("js-bind");
		path.set_extension("toml");
		let err_msg = format!("Couldn't read file 'js-bind.toml' at path {:?}", &path);
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Self::from_toml_config_str(&config_str)?)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConfigLock {
	functions: Vec<Function>,
}

const BEGINNING_LOCK_MSG: &str = r##"
# WARNING: This file is automatically @generated by `js-bind` project!
# Edit at your own risk!
"##;

impl ConfigLock {
	pub fn from_toml_config_str(string: &str) -> anyhow::Result<Self> {
		toml::from_str(string).context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}

	pub fn from_config_dir(path: &PathBuf) -> anyhow::Result<Self> {
		let mut path = path.clone();
		path.push("js-bind-lock");
		path.set_extension("toml");

		let err_msg = format!("Couldn't read file 'js-bind-lock.toml' at path {:?}", &path);
		// Check if exists
		if !path.exists() {
			// Make empty file
			std::fs::write(
				&path,
				&BEGINNING_LOCK_MSG,
			)
			.context("Failed to write to empty js-bind-lock.toml file")?;
		}
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Self::from_toml_config_str(&config_str)?)
	}

	fn write_at_dir(&self, path: &PathBuf) -> anyhow::Result<()> {
		let config_str = toml::to_string_pretty(&self).context("Failed to serialize ConfigLock")?;
		let data = format!("{}{}", BEGINNING_LOCK_MSG, config_str);
		std::fs::write(path, data).context("Failed to write to js-bind-lock.toml file")?;
		Ok(())
	}

	/// Appends the specified [Function] to the end of the [ConfigLock] file.
	pub fn append_func(&mut self, dir: &PathBuf, func: Function) -> anyhow::Result<()> {
		self.functions.push(func);
		self.write_at_dir(dir).context("Couldn't write at dir when adding function")
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Function {
	name: String,
	#[serde(rename = "mod")]
	mod_name: String,

	// timestamp: i64,
}

impl Function {
	pub fn new(name: String, mod_name: String) -> Self {
		// let timestamp = chrono::Utc::now().timestamp_millis();
		Self {
			name,
			mod_name,
			// timestamp,
		}
	}
}
