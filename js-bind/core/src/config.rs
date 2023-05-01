use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
	pub build: Build,
	pub modes: HashMap<String, Mode>,
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct Build {
	#[serde(rename = "output-dir")]
	pub output_dir: String,
	pub codegen: CodeGenBuild,
}

impl Default for Build {
	fn default() -> Self {
		Self {
			output_dir: "js".to_owned(),
			codegen: CodeGenBuild::default(),
		}
	}
}

/// Represents the [build.codegen] part of the config
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct CodeGenBuild {
	pub ts: bool,

	#[serde(rename = "bundle-name")]
	pub bundle_name: String,

	#[serde(rename = "rollup-config")]
	pub rollup_config: String,

	#[serde(rename = "npm-driver")]
	pub npm_driver: String,
}

/// ```toml
/// [build]
/// output-dir = "js"
///
/// [build.codegen]
/// ts = true # implied
/// bundle-name = "bundle" # no ext
/// rollup-config = "rollup.config.js"
/// npm-driver = "pnpm"
/// ```
///
/// ```rust
/// use js_bind_core::config::*;
/// let string = r##"
/// [build]
/// output-dir = "js"
///
/// [build.codegen]
/// 	ts = true # implied
/// 	bundle-name = "bundle" # no ext
/// 	rollup-config = "rollup.config.js"
/// 	npm-driver = "npm"
/// "##;
///
/// let de: Config = toml::from_str(string).expect("to work");
///
/// assert_eq!(de, Default::default())
/// ```
impl Default for CodeGenBuild {
	fn default() -> Self {
		Self {
			ts: true,
			bundle_name: "bundle".to_owned(),
			rollup_config: "rollup.config.js".to_owned(),
			npm_driver: "npm".to_owned(),
		}
	}
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
	pub fn from_config_dir(path: &PathBuf) -> anyhow::Result<Self> {
		let mut path = path.clone();
		path.push("js-bind");
		path.set_extension("toml");
		let err_msg = format!("Couldn't read file 'js-bind.toml' at path {:?}", &path);
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		Ok(Self::from_toml_config_str(&config_str)?)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Function {
	name: String,
	#[serde(rename = "mod")]
	mod_name: String,
}

impl Function {
	pub fn new(name: String, mod_name: String) -> Self {
		Self { name, mod_name }
	}
}
