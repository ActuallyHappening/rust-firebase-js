#![allow(unused_imports)]

use anyhow::{anyhow, Context};
use cargo_toml::Manifest;
use derive_new::new;
use derive_syn_parse::Parse;
use std::{default, unimplemented};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use serde::Deserialize;
use syn::{
	parse::{Parse, ParseStream},
	spanned::Spanned,
	token, Attribute, Expr, ExprLit, Ident, Item, ItemFn, ItemUse, Lit, LitStr, Meta, Token,
};

#[derive(Debug, Clone, Deserialize, new)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Config {
	pub template: String,

	/// Package name to replace with `crate`
	#[serde(default)]
	pub replace_package: Option<String>,

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
	/// inline_config
	config_ident: Ident,

	#[paren]
	/// ()
	paren: token::Paren,

	/// template = ""
	#[inside(paren)]
	config_template: InlineConfigTemplate,

	/// ,
	#[inside(paren)]
	comma: Option<Token![,]>,

	/// replace_package = ""
	#[inside(paren)]
	#[parse_if(comma.is_some())]
	config_replace_package: Option<InlineConfigReplacePackage>,
}

// impl ConfigParse {
// 	pub fn ident_eq_or(name: &str) -> impl FnOnce(ParseStream) -> syn::Result<Ident> {
// 		let name = name.to_string();
// 		move |input| {
// 			let ident = input.parse::<Ident>()?;
// 			if ident.to_string() != name {
// 				return Err(syn::Error::new(
// 					ident.span(),
// 					format!("Expected `eq`, not `{}`", ident),
// 				));
// 			}
// 			Ok(ident)
// 		}
// 	}
// }

#[derive(Debug, Clone, Parse)]
#[allow(dead_code)]
pub struct InlineConfigTemplate {
	/// template
	template_ident: Ident,
	/// = "..."
	#[prefix(Token![=])]
	template_value: LitStr,
}

#[derive(Debug, Clone, Parse)]
#[allow(dead_code)]
pub struct InlineConfigReplacePackage {
	/// replace_package
	package_ident: Ident,
	/// = "..."
	#[prefix(Token![=])]
	package_value: LitStr,
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
		fn confirm_ident(ident: &Ident, name: &str) -> syn::Result<()> {
			if ident.to_string() != name {
				Err(syn::Error::new(
					ident.span(),
					format!("Expected {}, not {}", name, ident),
				))
			} else {
				Ok(())
			}
		}
		confirm_ident(&config_parse.config_ident, "inline_config")?;

		confirm_ident(
			&config_parse.config_template.template_ident,
			"template",
		)?;

		if let Some(replace_package) = &config_parse.config_replace_package {
			confirm_ident(&replace_package.package_ident, "replace_package")?;
		}

		Ok(Self {
			template: config_parse.config_template.template_value.value(),
			replace_package: match config_parse.config_replace_package {
				Some(InlineConfigReplacePackage {
					package_value: ref lit,
					..
				}) => Some(lit.value()),
				_ => None,
			},
			parsed: Some(config_parse),
		})
	}
}

impl Parse for Config {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(input.parse::<ConfigParse>()?.try_into()?)
	}
}

#[derive(Debug, Clone)]
pub struct CodeBlock<T: Clone> {
	pub inner_lines: Vec<String>,
	pub meta: T,
}

#[derive(Default, Debug, Clone)]
pub struct NoCommentMeta {}

#[derive(Debug, Clone)]
pub struct CommentMeta {
	pub name: String,
}

#[derive(Debug, Clone)]
pub struct ProcessedMeta {
	pub meta: CommentMeta,
	pub processed: ItemFn,
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

	/// Process the CodeBlock, taking into account `replace-package`
	/// and other config options.
	pub fn process(self, config: &Config) -> syn::Result<CodeBlock<ProcessedMeta>> {
		let mut self_mut = self;
		// parse as fn
		let parsed: ItemFn = syn::parse_str(&self.inner_lines.join("\n"))?;

		// replace package name
		if let Some(name) = config.replace_package {
			self_mut = parsed.block.stmts.into_iter().map(|line| {
				if line.trim().starts_with("use ") && line.trim().contains(&name) && line.trim().ends_with(";") {
					let parsed_line = syn::parse_str::<ItemUse>(&line).expect("to parse as ItemUse");
					match syn::parse_str::<ItemUse>(&line) {
						Ok(mut import) => {
							match import.tree {
								syn::UseTree::Path(ref mut path) => {
									// get first ident
									let first_ident = path.ident.to_string();
									if first_ident == name {
										path.ident = syn::Ident::new(
											"crate",
											path.ident.span(),
										);
										// path.ident = Token![crate](path.ident.span());
									}
								}
								_ => unimplemented!("Only path imports are supported"),
							}
							import.to_token_stream().to_string()
						}
						Err(err) => {
							// eprintln!("Error parsing line as rust import. This may not be a fatal error, but often it is an early indicator of bad syntax\nErr:\n{:?}\nline: {:?}", err, line_mut)
							line
						}
					}
				} else {
					line
				}
			});
		}

		Ok(CodeBlock {
			inner_lines: self.inner_lines,
			meta: ProcessedMeta {
				meta: self.meta,
				processed: parsed,
			},
		})

		// unimplemented!()
	}
}

impl CodeBlock<ProcessedMeta> {
	pub fn into_tokens(self, config: &Config) -> syn::Result<ItemFn> {
		let template = config.interpolate_template(&self.inner_lines.join("\n"), &self.meta.meta.name);

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
	let span = raw_input.span();
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

	Ok(quote_spanned! {span=>
		#(#processed)*
	})
}

pub fn extract_doctests_impl(
	raw_attrs: TokenStream,
	raw_input: TokenStream,
) -> syn::Result<TokenStream> {
	let config = match Config::from_raw_input(raw_attrs.clone()) {
		Some(Ok(config)) => config,
		#[allow(unused_mut)]
		Some(Err(mut err)) => {
			// let extra_err = syn::Error::new_spanned(
			// 	raw_attrs.clone(),
			// 	"Failed to parse attributes as Config; \
			// 	#[extract_doctest] checks for inline configuration declerations before reading your Cargo.toml file from disk, \
			// 	but the inline configuration was not valid (exact error above).",
			// );
			// err.combine(extra_err);
			return Err(err);
		}
		None => Config::from_current_package().map_err(|e| {
			syn::Error::new_spanned(
				raw_attrs.clone(),
				format!(
					"Failed to parse Cargo.toml metadata as Config: \
					{:?} \
					note: no inline attribute configuration was found",
					e
				),
			)
		})?,
	};

	let tests = extract_doctests(&config, raw_input.clone())?;

	let span = raw_input.span();
	let expanded: TokenStream = quote_spanned! {span=>
		#raw_input

		#tests
	};

	return Ok(expanded);
}
