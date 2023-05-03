use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf};

/// ```rust
/// use js_bind_core::config::*;
/// // let string = include_str!("../../js-bind.toml");
/// // print cwd
/// println!("cwd: {}", std::env::current_dir().unwrap().to_str().unwrap());
/// let string = std::fs::read_to_string("../../js-bind-tests/js-bind.toml".to_string()).expect("Couldn't read file");
/// let config = toml::from_str::<Config>(string.as_str());
/// if config.is_err() {
/// 	panic!("Couldn't parse config: {:#?}", config);
/// }
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
	pub bundles: Vec<Bundles>,
	pub codegen: CodeGen,

	#[serde(skip)]
	full_path: PathBuf
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bundles {
	#[serde(rename = "if")]
	pub if_feature: String,
	#[serde(rename = "then")]
	pub then_path: String,
	#[serde(rename = "to-build")]
	pub to_build_command: String,	
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeGen {
	pub output: String,
	pub templates: Vec<Template>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Template {
	pub name: String,
	pub template: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct ConfigLock {
	#[serde(skip)]
	full_path: Option<PathBuf>,

	pub templates: Vec<LockTemplate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LockTemplate {
	#[serde(rename = "ref")]
	pub template_name_ref: String,
	#[serde(rename = "var-name")]
	pub var_name: String,
	#[serde(rename = "var-mod")]
	pub var_module: String,
}

const BEGINNING_LOCK_MSG: &str = r##"
# WARNING: This file is automatically @generated by `js-bind` project!
# Edit at your own risk!
"##;

pub trait FromTOMLCwd: Sized + serde::de::DeserializeOwned + serde::Serialize {
	fn from_cwd() -> anyhow::Result<Self> {
		let cwd = std::env::current_dir().context("Couldn't get current working directory")?;
		Self::from_config_dir(&cwd)
	}

	fn from_config_dir(path: &PathBuf) -> anyhow::Result<Self> {
		let path = Self::from_dir_to_file(path);

		let err_msg = format!("Couldn't read file at path {:?}", &path);
		// Check if exists
		if !path.try_exists().context(format!(
			"Can't tell if file exists as path {:?}",
			&path
		))? {
			// Make empty file
			// Self::default()
			// 	.write_at_file(&path)
			// 	.context("Couldn't write default empty file")?;
			// std::fs::write(&path, "");
		}
		let config_str = std::fs::read_to_string(&path).context(err_msg)?;
		let mut data = Self::from_toml_config_str(&config_str)?;
		data.set_full_path(path);
		Ok(data)
	}

	fn set_full_path(&mut self, path: PathBuf);

	fn from_toml_config_str(string: &str) -> anyhow::Result<Self> {
		toml::from_str(string).context("Couldn't parse config str")
		// .expect(&format!("Failed to parse config string:\n {:?}", string))
	}

	fn write_at_file(&self, file_path: &PathBuf) -> anyhow::Result<()> {
		let config_str = toml::to_string_pretty(&self).context("Failed to serialize ConfigLock")?;
		let data = format!("{}{}", BEGINNING_LOCK_MSG, config_str);
		std::fs::write(file_path, data).context(format!(
			"Failed to write to file at path: {:?}",
			&file_path
		))?;
		Ok(())
	}

	fn from_dir_to_file(dir: &PathBuf) -> PathBuf;
}

impl FromTOMLCwd for ConfigLock {
	fn set_full_path(&mut self, path: PathBuf) {
			self.full_path = Some(path);
	}
	fn from_dir_to_file(dir: &PathBuf) -> PathBuf {
		let mut path = dir.clone();
		path.push("js-bind");
		path.set_extension("lock");
		path
	}
}

impl ConfigLock {
	/// Appends the specified [Template] to the end of the [ConfigLock] file.
	/// The [bool] returned indicates if any changes were needed to be written to file.
	fn append_template(&mut self, dir: &PathBuf, template: LockTemplate) -> anyhow::Result<bool> {
		// Check if duplicate exists, if it does return
		if self.templates.iter().any(|f| f == &template) {
			return Ok(false);
		}
		self.templates.push(template);
		self
			.write_at_file(&Self::from_dir_to_file(dir))
			.context("Couldn't write at dir when adding template")?;
		Ok(true)
	}
}

impl FromTOMLCwd for Config {
	fn set_full_path(&mut self, path: PathBuf) {
		self.full_path = path;
	}
	fn from_dir_to_file(dir: &PathBuf) -> PathBuf {
		let mut path = dir.clone();
		path.push("js-bind");
		path.set_extension("toml");
		path
	}
}