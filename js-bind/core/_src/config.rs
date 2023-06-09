use anyhow::Context;
use derive_new::new;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::path::PathBuf;

use crate::docs::Docs;

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
#[derive(Debug, Clone, Deserialize, Serialize, new)]
pub struct Config {
	pub bundles: Vec<Bundle>,
	pub codegen: CodeGen,

	#[serde(skip)]
	full_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bundle {
	/// Feature namej
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
///
/// #templates = []
///
/// [[templates]]
/// name = "js_bind"
/// matches-wasmbindgen-import-signature = [{ empty = true }]
/// codegen-template = "NA"
/// documentation-template = "NA"
/// "#;
/// let config = toml::from_str::<CodeGen>(toml_str);
/// config.expect("to parse");
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeGen {
	pub output: String,
	#[serde(flatten)]
	pub templates: Templates,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Templates {
	pub templates: Vec<Template>,
}

#[derive(Debug, Clone, SmartDefault, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Template {
	#[default("NA")]
	pub name: String,

	#[serde(flatten)]
	#[serde(rename = "matches-wasmbindgen-signature")]
	pub matches_signature: Matches,

	#[serde(rename = "codegen-template")]
	pub codegen_template: String,
	#[serde(rename = "documentation-template")]
	pub documentation_template: String,

	#[serde(rename = "testgen-tempalte")]
	pub testgen_template: Testgen,
}

impl Template {
	/// Takes a rust function name and returns the js name,
	/// typically this changes the case of the name
	pub fn resolve_js_name(&self, rust_name: &str) -> String {
		// TODO: Implement case changing
		rust_name.to_string()
	}
}

/// Information which, when paired with func namd and mod name,
/// can create [LockTests].
#[derive(Debug, Clone, SmartDefault, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Testgen {
	#[default("NA")]
	template: String,
	#[serde(flatten)]
	pub specifics: TestgenSpecifics,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TestgenSpecifics {
	// #[serde(rename = "testgen-template-specifics")]
	pub specifics: Vec<TestgenSpecific>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TestgenSpecific {
	#[serde(rename = "name-suffix")]
	pub name_suffix: String,
	#[serde(rename = "specific-value")]
	#[serde(default)]
	pub var_specific_value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Matches {
	#[serde(rename = "matches-wasmbindgen-import-signature")]
	pub matches: Vec<Match>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Match {
	pub empty: Option<bool>,
	// pub attribute: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct ConfigLock {
	#[serde(skip)]
	full_path: Option<PathBuf>,

	pub templates: Vec<LockTemplate>,
}

/// Represents a template for a single function that is ready to expand, suitable to put into lockfile or
/// pass into function that expands it at build time
#[derive(Debug, Clone, SmartDefault, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LockTemplate {
	#[default("NA")]
	#[serde(rename = "codegen")]
	pub codegen: String,

	#[default("NA")]
	#[serde(rename = "documentation")]
	pub documentation: String,

	#[serde(flatten)]
	pub testgen: LockTests,
}

impl LockTemplate {
	pub fn new_from_template(template: Template, tests: LockTests, var_name: String, var_module: String) -> Self {
		Self {
			codegen: template.codegen_template,
			documentation: template.documentation_template,
			testgen: tests,
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LockTests {
	pub tests: Vec<LockTest>,
}

impl LockTests {
	/// Expands a [Testgen] template into [LockTests]
	pub fn new_from_template(matching_template: Testgen, docs: Docs, var_name: String, var_mod: String) -> LockTests {
		let template = matching_template;
		template.specifics.iter().map(|specific| {
			let mut test = LockTest::default();
			test.relative_file_name = format!("{}.test.ts", var_name);
			test.code = template.template.clone();
			test.code = test.code.replace("{{#name}}", var_name.as_str());
			test.code = test.code.replace("{{#mod}}", var_mod.as_str());
			test.code = test.code.replace("{{#specific}}", specific.var_specific_value.as_str());
			test
		}).collect();

		unimplemented!()
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, new)]
pub struct LockTest {
	#[serde(rename = "path")]
	pub relative_file_name: String,
	pub code: String,
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
		if !path
			.try_exists()
			.context(format!("Can't tell if file exists as path {:?}", &path))?
		{
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
		std::fs::write(file_path, data)
			.context(format!("Failed to write to file at path: {:?}", &file_path))?;
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
	/// Appends the specified [LockTemplate] to the end of the [ConfigLock] file.
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
