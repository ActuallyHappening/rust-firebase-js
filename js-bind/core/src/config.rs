use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use smart_default::SmartDefault;

/// ```rust
/// use js_bind_core::config::*;
/// // let string = include_str!("../../js-bind.toml");
/// let string = std::fs::read_to_string("../../../js-bind.toml".to_string()).expect("Couldn't read file");
/// let config: Config = toml::from_str(string.as_str()).expect("to work");
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
	pub build: Build,
	pub modes: HashMap<String, Mode>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Target {
	Web,
	Node,
}

impl Config {
	pub fn from_toml_config_str(string: &str) -> anyhow::Result<Self> {
		toml::from_str(string).context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}
	pub fn from_config_dir(path: &PathBuf) -> anyhow::Result<Self> {
		let mut path = path.clone();
		path.push("js-bind");
		path.set_extension("toml");
		let err_msg = format!("Couldn't read file 'js-bind.toml' at path {:?}", &path);
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Self::from_toml_config_str(&config_str)?)
	}
}

/// Represents the [build] part of the config
/// ```toml
/// [build]
/// output-dir = "js"
/// 
/// [build.codegen]
/// # see CodeGenBuild
/// ```
/// [CodeGenBuild]
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Build {
	pub codegen: CodeGenOptions,
	pub target: Targets,
}

#[derive(Debug, Hash, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Targets {
	pub node: CodeGenOptionsNode,
	pub web: CodeGenOptionsWeb,
}

#[derive(Debug, Hash, SmartDefault, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct CodeGenOptions {
	#[default(true)]
	pub ts: bool,

	#[serde(rename = "npm-driver")]
	#[default("npm")]
	pub npm_driver: String,
}


/// Represents the [build.codegen] part of the config
#[derive(Debug, Hash, SmartDefault, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct CodeGenOptionsNode {
	#[serde(rename = "bundle-name")]
	#[default("bundle-cjs.js")]
	pub bundle_name: String,

	#[serde(rename = "rollup-config")]
	#[default("js/node.config.js")]
	pub rollup_config: String,

	#[serde(rename = "feature-flag")]
	#[default("link-node")]
	pub feature_flag: String,
}

/// Represents the [build.codegen] part of the config
#[derive(Debug, Hash, SmartDefault, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct CodeGenOptionsWeb {
	#[serde(rename = "bundle-name")]
	#[default("bundle-esm.js")]
	pub bundle_name: String,

	#[serde(rename = "rollup-config")]
	#[default("js/web.config.js")]
	pub rollup_config: String,

	#[serde(rename = "feature-flag")]
	#[default("link-web")]
	pub feature_flag: String,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Mode {
	#[serde(rename = "mod")]
	pub mod_name: String,

	#[serde(rename = "type")]
	pub item_type: String,
	
	// #[serde(rename = "js-export-type")]
	// js_casing: String,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
enum ItemType {
	#[serde(rename = "function")]
	Function
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct ConfigLock {
	#[serde(default)]
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
		let path = Self::from_dir_to_file(path);

		let err_msg = format!("Couldn't read file 'js-bind.lock' at path {:?}", &path);
		// Check if exists
		if !path.try_exists().context(format!(
			"Can't tell if js-bind.lock file exists as path {:?}",
			&path
		))? {
			// Make empty file
			Self::default()
				.write_at_file(&path)
				.context("Couldn't write default empty file")?;
		}
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Self::from_toml_config_str(&config_str)?)
	}

	fn write_at_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
		let config_str = toml::to_string_pretty(&self).context("Failed to serialize ConfigLock")?;
		let data = format!("{}{}", BEGINNING_LOCK_MSG, config_str);
		std::fs::write(file_path, data).context(format!(
			"Failed to write to js-bind.lock file at path: {:?}",
			&file_path
		))?;
		Ok(())
	}

	fn from_dir_to_file(dir: &PathBuf) -> PathBuf {
		let mut path = dir.clone();
		path.push("js-bind");
		path.set_extension("lock");
		path
	}

	/// Appends the specified [Function] to the end of the [ConfigLock] file.
	/// The [bool] returned indicates if any changes were needed to be written to file.
	pub fn append_func(&mut self, dir: &PathBuf, func: Function) -> anyhow::Result<bool> {
		// Check if duplicate exists, if it does return
		if self.functions.iter().any(|f| f == &func) {
			return Ok(false);
		}
		self.functions.push(func);
		self
			.write_at_file(&Self::from_dir_to_file(dir))
			.context("Couldn't write at dir when adding function")?;
		Ok(true)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Function {
	name: String,
	#[serde(rename = "mode-name")]
	mode_name: String,
}

impl Function {
	pub fn new(name: String, mode_name: String) -> Self {
		Self { name, mode_name }
	}
}
