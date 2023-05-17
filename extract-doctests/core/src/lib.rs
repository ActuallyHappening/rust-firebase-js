#![allow(unused_imports)]

use anyhow::{anyhow, Context};
use cargo_toml::Manifest;
use derive_new::new;
use derive_syn_parse::Parse;
use std::{default, unimplemented};

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use syn::{
	parse::Parse, spanned::Spanned, token, Attribute, Expr, ExprLit, Ident, Item, ItemFn, ItemUse,
	Lit, LitStr, Meta,
};

// #[derive(Debug, Clone, Deserialize)]
// #[serde(deny_unknown_fields)]
// struct Config {
// 	template: String,
// }

// fn extract_documentation(attrs: &Vec<Attribute>) -> Vec<String> {
// 	attrs
// 		.iter()
// 		.filter_map(|attr| {
// 			if let Meta::NameValue(meta_name_value) = &attr.meta {
// 				if meta_name_value.path.is_ident("doc") {
// 					if let Expr::Lit(ExprLit {
// 						lit: Lit::Str(doc), ..
// 					}) = &meta_name_value.value
// 					{
// 						return Some(doc.value());
// 					}
// 				}
// 			}
// 			None
// 		})
// 		.map(|line| {
// 			line
// 				.lines()
// 				.map(|line| line.to_string())
// 				.collect::<Vec<_>>()
// 		})
// 		.flatten()
// 		.collect()
// }

// #[derive(Debug)]
// #[allow(dead_code)]
// struct RawCodeBlock {
// 	first_line: String,
// 	inner_lines: Vec<String>,
// 	last_line: String,
// }

// impl RawCodeBlock {
// 	fn new(first_line: String, inner_lines: Vec<String>, last_line: String) -> Option<Self> {
// 		assert!(first_line.trim().starts_with("```"));
// 		assert!(last_line.trim().starts_with("```"));
// 		if inner_lines.len() == 0 {
// 			return None;
// 		}
// 		Some(Self {
// 			first_line,
// 			inner_lines,
// 			last_line,
// 		})
// 	}
// }

// fn parse_documentation(lines: Vec<String>, config: &Config) -> Vec<TestableCodeBlock> {
// 	// groups into lines that start with "```*" and end with "```"
// 	let mut groups: Vec<Vec<String>> = Vec::new();
// 	let mut current_group: Vec<String> = Vec::new();
// 	let mut in_code_block = false;
// 	for line in lines {
// 		if line.trim().starts_with("```") {
// 			current_group.push(line);
// 			if in_code_block {
// 				groups.push(current_group);
// 				current_group = Vec::new();
// 			}
// 			in_code_block = !in_code_block;
// 		} else if in_code_block {
// 			current_group.push(line);
// 		}
// 	}

// 	// println!("groups: {:?}", groups);

// 	// convert into TestableCodeBlock
// 	let mut raw_code_blocks = Vec::new();
// 	for group in groups {
// 		let first_line = group[0].clone();
// 		let last_line = group[group.len() - 1].clone();

// 		let inner_lines = group[1..group.len() - 1].to_vec();

// 		if let Some(raw_code_block) = RawCodeBlock::new(first_line, inner_lines, last_line) {
// 			raw_code_blocks.push(raw_code_block);
// 		}
// 	}

// 	// filter code blocks that don't start with ```rust and have their first line as `// JSBIND-TEST`
// 	raw_code_blocks
// 		.into_iter()
// 		.filter(|b| {
// 			if b.inner_lines.len() == 0 {
// 				return false;
// 			}
// 			if !b
// 				.inner_lines
// 				.first()
// 				.unwrap()
// 				.trim()
// 				.starts_with("// JSBIND-TEST")
// 			{
// 				return false;
// 			}
// 			true
// 		})
// 		.map(|b| {
// 			let code = b.inner_lines[1..].to_vec();
// 			let name = b
// 				.inner_lines
// 				.first()
// 				.unwrap()
// 				.replace("// JSBIND-TEST", "")
// 				.trim()
// 				.to_string();
// 			assert_ne!(name, "", "Test name cannot be empty");

// 			TestableCodeBlock { code, name }.replace_crate_imports(&config)
// 		})
// 		.collect()
// }

// #[derive(Debug)]
// struct TestableCodeBlock {
// 	/// Lines of code to be subbed in for #code var
// 	code: Vec<String>,
// 	/// Full name of test
// 	name: String,
// 	// flags: Vec<Flag>,
// }

// impl TestableCodeBlock {
// 	pub fn into_tokens(&self, config: &Config) -> syn::Result<TokenStream> {
// 		let mut template = config.template.clone();
// 		template = template.replace("#code", self.code.join("\n").as_str());
// 		template = template.replace("#test_name", self.name.as_str());

// 		let mut tokens = TokenStream::new();

// 		// extend tokens with line
// 		// println!("Parsing line: {:?}", template);
// 		tokens.extend(syn::parse_str::<TokenStream>(&template)?);

// 		Ok(tokens)
// 	}

// 	/// Replaces potential imports to absolute package name to `crate`
// 	pub fn replace_crate_imports(self, config: &Config) -> Self {
// 		let code = self.code.into_iter().map(|line| {
// 				let mut line_mut = line;
// 				if line_mut.trim().starts_with("use ") && line_mut.trim().ends_with(";") {
// 					// parse line as rust import `use foo::bar;`
// 					match syn::parse_str::<ItemUse>(&line_mut) {
// 						Ok(import) => {
// 							let mut import = import;
// 							match import.tree {
// 								syn::UseTree::Path(ref mut path) => {
// 									// get first ident
// 									let first_ident = path.ident.to_string();
// 									if first_ident == config.replace_package.clone().expect("to have a replace_package") {
// 										path.ident = syn::Ident::new(
// 											"crate",
// 											path.ident.span(),
// 										);
// 										// path.ident = Token![crate](path.ident.span());
// 									}
// 								}
// 								_ => unimplemented!("Only path imports are supported"),
// 							}
// 							line_mut = import.to_token_stream().to_string();
// 						}
// 						Err(err) => {
// 							eprintln!("Error parsing line as rust import. This may not be a fatal error, but often it is an early indicator of bad syntax\nErr:\n{:?}\nline: {:?}", err, line_mut)
// 						}
// 					}
// 				}
// 				line_mut
// 			}).collect();
// 		Self {
// 			code,
// 			name: self.name,
// 		}
// 	}
// }

// /// Generates test items suitable to be ran by `wasm-bindgen-test`
// ///
// /// ## Examples
// /// Correct example:
// /// ```rust
// /// use js_bind_core::config::DocTestGen;
// /// use js_bind_core::macros::gen_doc_test;
// /// use quote::ToTokens;
// ///
// /// let toml_str = r##"
// /// replace-package = "firebase_js_sys"
// /// template = """
// /// #[::wasm_bindgen_test::wasm_bindgen_test]
// /// fn #test_name() {
// /// 	#code
// /// }
// /// """
// /// "##;
// ///
// /// let config: DocTestGen = toml::from_str(toml_str).expect("to parse");
// ///
// /// let attributes1 = vec![
// ///		syn::parse_quote!{ #[doc = r#"
// ///		Example test:
// /// 	```rust
// /// 		// JSBIND-TEST example_test_name
// /// 		assert_eq!(1, 1);
// /// 	```
// /// "#]}
// /// ];
// /// let expected1 = quote::quote!{
// /// 	#[::wasm_bindgen_test::wasm_bindgen_test]
// /// 	fn example_test_name() {
// /// 		assert_eq!(1, 1);
// /// 	}
// /// };
// /// let generated1 = gen_doc_test(&config, &attributes1).expect("there to be a test");
// /// assert_eq!(expected1.to_string(), generated1.to_token_stream().to_string());
// ///
// /// let attributes2 = vec![syn::parse_quote!{ #[doc = r#"
// /// 	Example test that shows off package import replacement:
// /// 	```rust
// /// 	// JSBIND-TEST example_test_with_replaced_package
// /// 	use firebase_js_sys::something::deep as here;
// /// 	assert_eq!(42, 42);
// /// 	```
// /// "#]}];
// /// let expected2 = quote::quote!{
// /// 	#[::wasm_bindgen_test::wasm_bindgen_test]
// /// 	fn example_test_with_replaced_package() {
// /// 		// Note how the package name is replaced, because this test
// /// 		// is not a documentation test this will ususally work
// /// 		use crate::something::deep as here;
// /// 		assert_eq!(42, 42);
// /// 	}
// /// };
// /// let generated2 = gen_doc_test(&config, &attributes2).expect("there to be a test");
// /// assert_eq!(expected2.to_string(), generated2.to_token_stream().to_string());
// /// ```
// ///
// /// Example that doesn't produce valid rust code:
// /// ```rust,should_panic
// /// use js_bind_core::config::DocTestGen;
// /// use js_bind_core::macros::gen_doc_test;
// /// use quote::ToTokens;
// ///
// /// let toml_str = r##"
// /// web-feature-name = "example-flag"
// /// template = """
// /// #[::wasm_bindgen_test::wasm_bindgen_test]
// /// fn #test_name() {
// /// 	assert_eq!(#web_feature_name, "example-flag");
// /// 	#code
// /// }
// /// """
// /// "##;
// ///
// /// let config: DocTestGen = toml::from_str(toml_str).expect("to parse");
// ///
// /// let attributes1 = vec![
// /// 	syn::parse_quote!{ #[doc = r#"
// /// 	Example test:
// /// 	```rust
// /// 		// JSBIND-TEST example_test_name
// /// 		assert_eq!(1, 1);
// ///
// /// 		this_is_not valid- rust_code ();
// /// 	```
// /// "#]}
// /// ];
// ///
// /// gen_doc_test(&config, &attributes1);
// /// ```
// pub fn _gen_doc_test(config: &Config, attrs: &Vec<Attribute>) -> Option<ItemFn> {
// 	let documentation = extract_documentation(attrs);
// 	// println!("documentation: {:?}", documentation);

// 	let testable_code_blocks = parse_documentation(documentation, &config);
// 	// println!("testable_code_blocks: {:?}", testable_code_blocks);

// 	let tokens: TokenStream = testable_code_blocks
// 		.iter()
// 		.map(|b| b.into_tokens(&config))
// 		.fold(TokenStream::new(), |mut acc, f| {
// 			acc.extend(f);
// 			acc
// 		});
// 	// .collect();

// 	// to_debug_file("debug-docgen.rs", &tokens.clone());

// 	match syn::parse2(tokens.clone()) {
// 		Ok(item_fn) => Some(item_fn),
// 		Err(_) => {
// 			// println!("Error parsing tokens: {:?}", err);
// 			if !tokens.is_empty() {
// 				panic!(
// 					r#"Error parsing tokens as a rust function, make sure your template produces valid rust code: "
// {:?}
// ""#,
// 					tokens.to_string()
// 				);
// 			}
// 			None
// 		}
// 	}
// }

#[derive(Debug, Clone, Deserialize, new)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Config {
	pub template: String,

	#[serde(skip)]
	#[new(default)]
	parsed: Option<ConfigParse>,
}

/// Lower level parsable version of config for attribute parsing purposes
/// Parses config like:
/// ```rust
/// use extract_doctests_core::ConfigParse;
/// use syn::parse_quote;
///
/// let config: ConfigParse = parse_quote!{inline_config(template = r##"foobar"##)};
/// ```
#[derive(Debug, Clone, Parse)]
#[allow(dead_code)]
pub struct ConfigParse {
	inline_config_ident: Ident,
	#[paren]
	inline_config_paren: token::Paren,
	#[inside(inline_config_paren)]
	inline_config: InlineConfig,
}

#[derive(Debug, Clone, Parse)]
#[allow(dead_code)]
pub struct InlineConfig {
	template: Ident,
	eq_sign: token::Eq,
	template_value: LitStr,
}

impl Config {
	pub fn from_current_package() -> anyhow::Result<Config> {
		let dir = std::env::var("CARGO_MANIFEST_DIR")?;
		let path = std::path::Path::new(&dir).join("Cargo.toml");
		let manifest = cargo_toml::Manifest::from_path(path)?;

		#[derive(Deserialize)]
		struct RawConfig {
			#[serde(rename = "extract-doctests")]
			config: Config,
		}

		let metadata: RawConfig = manifest
			.package
			.ok_or(anyhow!("Cargo.toml does not contain package entry"))?
			.metadata
			.ok_or(anyhow!(
				"Cargo.toml does not contain a package.metadata entry"
			))?
			// deserialize into Config
			.try_into()
			.context("Couldn't parse Cargo.toml>package.metadata")?;

		Ok(metadata.config)
	}

	pub fn from_raw_input(raw_input: TokenStream) -> Option<syn::Result<Self>> {
		if raw_input.is_empty() {
			None
		} else {
			Some(syn::parse2::<Self>(raw_input))
		}
	}

	pub fn interpolate_template(&self, var_code: &str, var_test_name: &str) -> String {
		let mut template = self.template.clone();
		template = template.replace("{code}", var_code);
		template = template.replace("{test_name}", var_test_name);
		template
	}
}

impl TryFrom<ConfigParse> for Config {
	type Error = syn::Error;

	fn try_from(config_parse: ConfigParse) -> syn::Result<Self> {
		config_parse.inline_config_ident
			.to_string()
			.as_str()
			.strip_prefix("inline_config")
			.ok_or_else(|| {
				syn::Error::new(
					config_parse.inline_config_ident.span(),
					format!("Expected inline_config, not {}", config_parse.inline_config_ident),
				)
			})?;
		
		config_parse.inline_config.template.to_string().as_str().strip_prefix("template").ok_or_else(|| {
			syn::Error::new(
				config_parse.inline_config.template.span(),
				format!("Expected template, not {}", config_parse.inline_config.template),
			)
		})?;

		Ok(Self {
			template: config_parse.inline_config.template_value.value(),
			parsed: Some(config_parse),
		})
	}
}

impl Parse for Config {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(input.parse::<ConfigParse>()?.try_into()?)
	}
}

#[derive(Debug)]
pub struct CodeBlock<T> {
	pub inner_lines: Vec<String>,
	pub meta: T,
}

#[derive(Default, Debug)]
pub struct NoCommentMeta {}

#[derive(Debug)]
pub struct CommentMeta {
	pub name: String,
}

impl CodeBlock<NoCommentMeta> {
	pub fn new(inner_lines: Vec<String>) -> Self {
		Self {
			inner_lines,
			meta: NoCommentMeta::default(),
		}
	}

	/// Extracts the documentation from a raw list of attributes
	///
	/// ```rust
	/// use extract_doctests_core::CodeBlock;
	/// use syn::parse_quote;
	///
	/// let attrs = vec![
	/// 	parse_quote!{ #[doc = r#"
	/// 	Example test:
	/// 	```rust
	/// 		// comment here
	/// 		assert_eq!(1, 2);
	/// 	```
	/// "#]}
	/// ];
	///
	/// let code_block = CodeBlock::from_attrs(&attrs).expect("to have a code block");
	/// assert_eq!(code_block.get(0).unwrap().inner_lines.iter().map(|l| l.trim()).collect::<Vec<_>>(), vec![
	/// 	"// comment here".to_string(),
	/// 	"assert_eq!(1, 2);".to_string(),
	/// ]);
	///
	/// let attrs2 = vec![
	/// 	parse_quote!{ #[doc = r#"```rust"#]},
	/// 	parse_quote!{ #[doc = r#"// comment here first"#]},
	/// 	parse_quote!{ #[doc = r#"assert_eq!(1, 1);"#]},
	/// 	parse_quote!{ #[doc = r#"```"#]},
	///
	/// 	parse_quote!{ #[doc = r#"Some documentation"#]},
	///
	/// 	parse_quote!{ #[doc = r#"```rust"#]},
	/// 	parse_quote!{ #[doc = r#"// comment here again"#]},
	/// 	parse_quote!{ #[doc = r#"assert_eq!(42, 69);"#]},
	/// 	parse_quote!{ #[doc = r#"```"#]},
	/// ];
	///
	/// let code_block = CodeBlock::from_attrs(&attrs2).expect("to have code blocks");
	/// println!("code_block: {:?}", code_block);
	/// assert_eq!(code_block.len(), 2);
	/// 
	/// assert_eq!(code_block.get(0).unwrap().inner_lines.iter().map(|l| l.trim()).collect::<Vec<_>>(), vec![
	/// 	"// comment here first".to_string(),
	/// 	"assert_eq!(1, 1);".to_string(),
	/// ]);
	/// 
	/// assert_eq!(code_block.get(1).expect("to have second block").inner_lines.iter()
	/// 	.map(|l| l.trim()).collect::<Vec<_>>(), vec![
	/// 	"// comment here again".to_string(),
	/// 	"assert_eq!(42, 69);".to_string(),
	/// ], "second block");
	/// ```
	pub fn from_attrs(attrs: &Vec<Attribute>) -> Option<Vec<CodeBlock<NoCommentMeta>>> {
		let mut raw_doc_lines: Vec<String> = Vec::new();
		for attr in attrs {
			if let Meta::NameValue(meta_name_value) = &attr.meta {
				if meta_name_value.path.is_ident("doc") {
					if let Expr::Lit(ExprLit {
						lit: Lit::Str(doc), ..
					}) = &meta_name_value.value
					{
						raw_doc_lines.extend(doc.value().lines().map(|l| l.to_string()))
					}
				}
			}
		}

		let mut code_blocks = Vec::new();
		let mut current_block = Vec::new();
		let mut in_code_block = false;
		for line in raw_doc_lines {
			if line.trim().starts_with("```") {
				if in_code_block {
					code_blocks.push(CodeBlock::new(current_block));
					current_block = Vec::new();
					in_code_block = false;
				} else {
					in_code_block = true;
				}
			} else if in_code_block {
				current_block.push(line.to_string());
			}
		}

		if code_blocks.len() == 0 {
			return None;
		} else {
			Some(code_blocks)
		}
	}

	/// Parses the potential code block into a `CodeBlock<CommentMeta>`
	///
	/// ```rust
	/// use extract_doctests_core::{CodeBlock, NoCommentMeta};
	///
	/// let code_block = <CodeBlock<NoCommentMeta>>::new(vec![
	/// 	" // extract-doctests example_test_name".to_string(),
	/// 	"assert_eq!(1, 1);".to_string(),
	/// ]);
	///
	/// let code_block = code_block.check_testable().expect("to have a testable code block");
	/// assert_eq!(code_block.inner_lines.iter().map(|l| l.trim()).collect::<Vec<_>>(), vec![
	/// 	"// extract-doctests example_test_name".to_string(),
	/// 	"assert_eq!(1, 1);".to_string(),
	/// ]);
	pub fn check_testable(self) -> Option<CodeBlock<CommentMeta>> {
		let first_line = self
			.inner_lines
			.first()
			.expect("Expected to have at least one line in documentation test")
			.clone();
		let stripped = first_line
			.trim()
			.trim_start_matches("#")
			.trim_start_matches("//")
			.trim();
		if !stripped.starts_with("extract-doctests") {
			return None;
		}
		let name = stripped
			.trim_start_matches("extract-doctests")
			.trim()
			.to_string();

		Some(<CodeBlock<CommentMeta>>::new_with_meta(
			self.inner_lines, //[1..].to_vec(),
			CommentMeta { name },
		))
	}
}

impl CodeBlock<CommentMeta> {
	pub fn new_with_meta(inner_lines: Vec<String>, meta: CommentMeta) -> Self {
		Self { inner_lines, meta }
	}

	pub fn into_tokens(self, config: &Config) -> syn::Result<ItemFn> {
		let template = config.interpolate_template(&self.inner_lines.join("\n"), &self.meta.name);

		// Validate template is an ItemFn
		match syn::parse_str::<ItemFn>(&template) {
			Ok(item_fn) => {
				// println!("item_fn: {:?}", item_fn);
				Ok(item_fn)
			}
			Err(err) => {
				let mut base_error = syn::Error::new_spanned(
					template,
					"Failed to parse template as a rust function. Make sure your template produces valid rust code after interpolation.",
				);
				base_error.combine(err);
				Err(base_error)
			}
		}
	}
}

/// Returns a vector for each item found, and within each item's vector
/// a vector containing the attributes of the item (ALL the attributes,
/// not just the doc ones)
///
/// TODO: Change to references for proper error bubbling
pub fn raw_into_processable_documentations(
	raw_input: TokenStream,
) -> syn::Result<Vec<Vec<Attribute>>> {
	// Parse input into syn::Item
	let input_span = raw_input.span();
	match syn::parse2::<Item>(raw_input) {
		Ok(item) => {
			match item {
				// gets attrs from extern {} block's functions
				Item::ForeignMod(foreign_mod) => {
					return Ok(
						foreign_mod
							.items
							.into_iter()
							.filter_map(|item| match item {
								syn::ForeignItem::Fn(foreign_fn) => Some(foreign_fn.attrs),
								_ => None,
							})
							.collect(),
					);
				}
				// gets attrs from free-standing function
				Item::Fn(item_fn) => {
					return Ok(vec![item_fn.attrs]);
				}
				// gets attrs from struct
				Item::Struct(item_struct) => {
					return Ok(vec![item_struct.attrs]);
				}
				// give error for unsupported type
				_ => Err(syn::Error::new(
					input_span,
					"#[extract_docs] The item passed to this macro is not yet implemented / supported.",
				)),
			}
		}
		Err(e) => {
			let mut base_err = syn::Error::new(
				input_span,
				"#[extract_docs] Failed to parse input as a rust item. \
				Make sure you are using this macro on a valid function, struct or extern block.",
			);
			base_err.combine(e);
			return Err(base_err);
		}
	}
}

/// Does the heavy lifting
///
/// ```rust
/// use extract_doctests_core::{Config, extract_doctests};
/// use quote::ToTokens;
/// use syn::parse_quote;
/// use quote::quote;
///
/// let config = Config::new(
/// 	r#"
/// 		fn {test_name}() {
/// 			{code}
/// 		}
/// 	"#.to_string(),
/// );
///
/// let input = quote!{
/// 	#[doc = r#"
/// 	Example test:
/// 	```rust
/// 		// extract-doctests example_test_name
/// 		assert_eq!(1, 1);
/// 	```
/// "#]
/// fn example() {}
/// };
///
/// let expected = quote!{
/// 	fn example_test_name() {
/// 		assert_eq!(1, 1);
/// 	}
/// };
///
/// let actual = extract_doctests(&config, input).expect("to have a test");
///
/// assert_eq!(expected.to_string(), actual.to_token_stream().to_string());
///
/// let input2 = quote!{
/// 	#[doc = r#"
/// 	Example test (that is not extracted):
/// 	```rust
/// 		assert_eq!(42, 42);
/// 	```
///
/// 	Example test that IS extracted:
/// 	```rust
/// 		// extract-doctests example_test_name_extracted
/// 		assert_eq!(1, 1);
/// 	```
/// "#]
/// fn example() {}
/// };
///
/// let expected2 = quote!{
/// 	fn example_test_name_extracted() {
/// 		assert_eq!(1, 1);
/// 	}
/// };
///
/// let actual2 = extract_doctests(&config, input2).expect("to have a test");
///
/// assert_eq!(expected2.to_string(), actual2.to_token_stream().to_string());
/// ```
pub fn extract_doctests(config: &Config, raw_input: TokenStream) -> syn::Result<TokenStream> {
	let processed: Vec<ItemFn> = raw_into_processable_documentations(raw_input)?
		.iter()
		// .inspect(|attrs| println!("Attrs: {:?}", attrs))
		.filter_map(CodeBlock::from_attrs)
		.flatten()
		// .inspect(|code_block| println!("code_block: {:?}", code_block))
		.filter_map(CodeBlock::check_testable)
		// .inspect(|code_block| println!("code_blocks processed: {:?}", code_block))
		.map(|code_block| code_block.into_tokens(config))
		.collect::<Result<_, _>>()?;

	Ok(quote! {
		#(#processed)*
	})
}

pub fn extract_doctests_impl(
	raw_attrs: TokenStream,
	raw_input: TokenStream,
) -> syn::Result<TokenStream> {
	let config = match Config::from_raw_input(raw_attrs.clone()) {
		Some(Ok(config)) => config,
		Some(Err(err)) => {
			let mut base_error = syn::Error::new_spanned(
				raw_attrs.clone(),
				"Failed to parse attributes as Config; \
				#[extract_doctest] checks for inline configuration declerations before reading your Cargo.toml file from disk, \
				but the inline configuration was not valid. The exact error is attached below.
				",
			);
			base_error.combine(err);
			return Err(base_error);
		}
		None => Config::from_current_package().map_err(|e| {
			syn::Error::new_spanned(
				raw_attrs.clone(),
				format!(
					"Failed to parse Cargo.toml metadata as Config; \
					#[extract_doctest] no inline attribute configuration was found. Error parsing config: \
					{:?}",
					e
				),
			)
		})?,
	};

	let tests = extract_doctests(&config, raw_input.clone())?;

	let expanded: TokenStream = quote! {
		#raw_input

		#tests
	};

	return Ok(expanded);
}
