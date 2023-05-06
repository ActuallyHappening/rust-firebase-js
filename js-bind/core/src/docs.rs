use std::str::FromStr;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use syn::parse::*;
use syn::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
struct CodeBlock {
	lang: Lang,
	options: Options,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
enum Lang {
	Rust(String),
	// TODO: Add JS testing support?
	Other(String),
}

impl FromStr for Lang {
	type Err = ();

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			"rust" => Ok(Self::Rust("rust".to_owned())),
			// "rs" => Ok(Self::Rust("rs".to_owned())), // TODO: Test
			_ => Ok(Self::Other(s.to_owned())),
		}
	}
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize, SmartDefault)]
enum Options {
	#[default]
	None,
	// Ignore,
	// ShouldPanic,
	// NoRun,
	// CompileFail,
}

impl FromStr for Options {
	type Err = ();

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s {
			// "ignore" => Ok(Self::Ignore),
			// "should_panic" => Ok(Self::ShouldPanic),
			// "no_run" => Ok(Self::NoRun),
			// "compile_fail" => Ok(Self::CompileFail),
			_ => Ok(Self::None),
		}
	}
}

impl CodeBlock {
	/// Gets the documentation comments from a function
	///
	/// TODO: Not clone all of [attrs]? maybe expensive?
	pub fn get_docs(attrs: &Vec<Attribute>) -> Vec<String> {
		let mut doc_comments = Vec::new();
		attrs.clone().into_iter().for_each(|attr| {
			if let Meta::NameValue(meta_name_value) = attr.meta {
				if meta_name_value.path.is_ident("doc") {
					match meta_name_value.value {
						Expr::Lit(ExprLit {
							lit: Lit::Str(doc), ..
						}) => {
							doc_comments.push(doc.value());
						}
						_ => {}
					}
				}
			}
		});
		doc_comments
	}

	pub fn get_code_blocks(docs: &Vec<String>) -> Vec<Vec<String>> {
		let mut code_blocks = Vec::new();
		let mut in_code_block = false;
		let mut code_block = Vec::new();
		for line in docs {
			let line = line.trim();
			if line.starts_with("```") {
				if in_code_block {
					// Exiting code block
					code_block.push(line.to_owned());
					code_blocks.push(code_block);
					code_block = Vec::new();
					in_code_block = false;
				} else {
					// Entering code block
					code_block = Vec::new(); // Repetition of line 4 above
					code_block.push(line.to_owned());
					in_code_block = true;
				}
			} else if in_code_block {
				// In code block, not boundary
				code_block.push(line.to_owned());
			}
		}
		code_blocks
	}

	pub fn parse_code_block(block: Vec<String>) -> CodeBlock {
		let first_line = block.get(0).expect("Code block has no lines");
		let last_line = block.get(block.len() - 1).expect("Code block has no lines");

		assert!(first_line.starts_with("```"));
		assert!(last_line.starts_with("```"));

		// Find all words, seperated by commans, like 'rust,ignore'
		let words = first_line
			.trim_start_matches("```")
			.trim()
			.split(",")
			.map(|s| s.trim())
			.filter(|s| !s.is_empty())
			.collect::<Vec<_>>();

		let mut lang: Lang = Lang::Rust("".to_owned());
		let mut options: Options = Options::None;
		if let Some(maybe_lang) = words.first() {
			if let Ok(_lang) = maybe_lang.parse() {
				lang = _lang;

				// parse second word
				if let Some(maybe_option) = words.get(1) {
					if let Ok(_option) = maybe_option.clone().parse() {
						options = _option;
					}
				}
			}
		}

		CodeBlock { lang, options }

		// unimplemented!()

		// let lang = first_line
		// 	.trim_start_matches("```")
		// 	.trim()
		// 	.parse::<Lang>()
		// 	.expect("Couldn't parse language identifier");

		// let mut options = Options::default();
		// if let Lang::Rust(_) = lang {
		// 	// parse extra option, seperated by comma
		// 	if let Some(option) = first_line
		// 		.trim_start_matches("```")
		// 		.trim()
		// 		.split(",")
		// 		.skip(1)
		// 		.map(|s| s.trim())
		// 		.filter(|s| !s.is_empty())
		// 		// get only first
		// 		.next()
		// 		.map(|s| s.parse::<Options>().expect("Couldn't parse options"))
		// 	{
		// 		options = option;
		// 	}
		// }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::*;
	use syn::*;
	use syn::parse::*;

	#[test]
	fn test_getting_codeblocks_from_fn() {
		let func = syn::parse2::<ItemFn>(quote!{
			/// Documentation
			/// ## Example
			/// ```rust
			/// fn main() {
			/// println!("First example");
			/// }
			/// ```
			/// ```rust,ignore
			/// fn main() {
			/// println!("Second example");
			/// }
			/// ```
			/// ```
			/// fn main() {
			/// println!("Hello, world!");
			/// }
			/// ```
			/// Finally, the 18th line
			pub fn test_func(input: String) -> bool { true }
		}).unwrap();
		let attrs = func.attrs;

		let docs = CodeBlock::get_docs(&attrs);
		assert_eq!(docs.len(), 18);
		assert_eq!(docs.get(0).unwrap().trim(), "Documentation");

		let code_blocks = CodeBlock::get_code_blocks(&docs);
		assert_eq!(code_blocks.len(), 3);
		assert_eq!(code_blocks.get(0).unwrap().len(), 5);
		assert_eq!(code_blocks.get(1).unwrap().get(2).unwrap().trim(), r#"println!("Second example");"#);

		let mut parsed_blocks = Vec::new();
		code_blocks.into_iter().for_each(|block| {
			let code_block = CodeBlock::parse_code_block(block);

			parsed_blocks.push(code_block);
			// eprintln!("Code block: {:#?}", code_block);
		});
		assert_eq!(parsed_blocks.len(), 3);
		assert_eq!(parsed_blocks.get(0).unwrap().lang, Lang::Rust("rust".to_owned()));
		assert_eq!(parsed_blocks.get(0).unwrap().options, Options::None);
		assert_eq!(parsed_blocks.get(1).unwrap().lang, Lang::Rust("rust".to_owned()));
		assert_eq!(parsed_blocks.get(1).unwrap().options, Options::None); // TODO: implement ignore
		assert_eq!(parsed_blocks.get(2).unwrap().lang, Lang::Rust("".to_owned()));
		assert_eq!(parsed_blocks.get(2).unwrap().options, Options::None); // here too
	}

	#[test]
	fn test_get_docs_specific_empty() {
		let func = quote!{
			pub fn test_func(input: String) -> bool { true }
		};
		let func = parse2::<ItemFn>(func).unwrap();
		let attrs = func.attrs;

		let docs = CodeBlock::get_docs(&attrs);

		assert_eq!(docs.len(), 0);
	}

	#[test]
	fn test_get_docs_specific1() {
		let func = quote!{
			/// Some documentation
			/// ## Example
			/// ```rust
			/// fn main() {
			/// println!("Hello, world!");
			/// }
			/// ```
			pub fn test_func(input: bool) -> &str { "maybe" }
		};
		let func = parse2::<ItemFn>(func).unwrap();
		let attrs = func.attrs;

		let docs = CodeBlock::get_docs(&attrs);

		assert_eq!(docs.len(), 7);
		assert_eq!(docs.get(0).unwrap().trim(), "Some documentation");
	}

	#[test]
	fn test_get_code_blocks_specific() {
		let lines = vec![
			"Some documentation",
			"## Example",
			"```rust",
			"fn main() {",
			"println!(\"Hello, world!\");",
			"}",
			"```",
			"Some more stuff",
			"```",
			"fn main() {",
			"println!(\"Hello, world!\");",
			"}",
			"// More lines",
			"```",
		].into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();

		let code_blocks = CodeBlock::get_code_blocks(&lines);

		assert_eq!(code_blocks.len(), 2);
		assert_eq!(code_blocks.get(0).unwrap().len(), 5);
		assert_eq!(code_blocks.get(1).unwrap().len(), 6);
		assert_eq!(code_blocks[0], vec![
			"```rust".to_owned(),
			"fn main() {".to_owned(),
			"println!(\"Hello, world!\");".to_owned(),
			"}".to_owned(),
			"```".to_owned(),
		]);
		assert_eq!(code_blocks[1], vec![
			"```".to_owned(),
			"fn main() {".to_owned(),
			"println!(\"Hello, world!\");".to_owned(),
			"}".to_owned(),
			"// More lines".to_owned(),
			"```".to_owned(),
		]);
	}

	#[test]
	fn test_parse_code_block_specific1() {
		let block = vec![
			"```rust,ignore".to_owned(),
			"fn main() {".to_owned(),
			"println!(\"Hello, world!\");".to_owned(),
			"}".to_owned(),
			"```".to_owned(),
		];

		let code_block = CodeBlock::parse_code_block(block);

		assert_eq!(code_block.lang, Lang::Rust("rust".to_owned()));
		assert_eq!(code_block.options, Options::None);
	}

	#[test]
	fn test_parse_code_block_specific2() {
		let block = vec![
			"```".to_owned(),
			"fn main() {".to_owned(),
			"println!(\"Hello, world!\");".to_owned(),
			"}".to_owned(),
			"```".to_owned(),
		];

		let code_block = CodeBlock::parse_code_block(block);

		assert_eq!(code_block.lang, Lang::Rust("".to_owned()));
		assert_eq!(code_block.options, Options::None);
	}

	#[test]
	fn test_parse_code_block_specific3() {
		let block = vec![
			"```js,unknown_attr".to_owned(),
			"fn main() {".to_owned(),
			"println!(\"Hello, world!\");".to_owned(),
			"}".to_owned(),
			"```".to_owned(),
		];

		let code_block = CodeBlock::parse_code_block(block);

		assert_eq!(code_block.lang, Lang::Other("js".to_owned()));
		assert_eq!(code_block.options, Options::None);
	}
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeContext {
// 	function_name: String,
// 	js_function_name: String,
// 	js_module_name: String,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeBlock {
// 	lines: Vec<String>,
// 	parameters: CodeParams,
// 	context: CodeContext,
// }

// impl CodeBlock {
// 	pub fn new(lines: Vec<String>, context: CodeContext) -> CodeBlock {
// 		Self {
// 			// TODO: not clone for performance?
// 			lines: lines.clone(),
// 			parameters: CodeParams::from_strings(lines),
// 			context,
// 		}
// 	}
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeParams {
// 	lang_identifier: Option<String>,
// 	test_option: Option<String>,
// 	other: Option<Vec<String>>
// }

// impl CodeParams {
// 	/// Transforms a string vector into a CodeParams struct
// 	///
// 	pub fn from_strings(start_line: String) -> CodeParams {
// 		let items: Vec<&str> = start_line.trim().to_owned().split(",").collect();
// 		Self {
// 			// TODO: Not clone for performance?
// 			lang_identifier: items.first().map(|f| f.to_owned()),
// 			test_option: items.get(1).cloned(),
// 			other: items.get(2..).map(|v| v.to_vec()),
// 		}
// 	}
// }

// fn _handle_doc(doc: Vec<String>) {
// 	// eprintln!("Doc: {:?}", doc);

// 	// Extract ```rs ``` code blocks
// 	let mut code_blocks: Vec<Vec<String>> = Vec::new();
// 	let mut in_code_block = false;
// 	let mut code_block: Vec<String> = Vec::new();
// 	for line in doc {
// 		let line = line.trim();
// 		if line.starts_with("```") {
// 			if in_code_block {
// 				// Exiting code block
// 				code_block.push(line.to_owned());
// 				code_blocks.push(code_block);
// 				code_block = Vec::new();
// 				in_code_block = false;
// 			} else {
// 				// Entering code block
// 				code_block = Vec::new(); // Repetition of line 4 above
// 				code_block.push(line.to_owned());
// 				in_code_block = true;
// 			}
// 		} else if in_code_block {
// 			// In code block, not boundary
// 			code_block.push(line.to_owned());
// 		}
// 	}

// 	eprintln!("Code blocks: {:#?}", code_blocks);
// }

// fn handle_doc_comments(func: &ItemFn) {
// 	// TODO: Not clone? maybe expensive?
// 	let mut doc_comments = Vec::new();
// 	(func.attrs.clone()).into_iter().for_each(|attr| {
// 		eprintln!("Attr: {:#?}", attr);
// 		if let Meta::NameValue(meta_name_value) = attr.meta {
// 			if meta_name_value.path.is_ident("doc") {
// 				match meta_name_value.value {
// 					Expr::Lit(ExprLit { lit: Lit::Str(doc), .. }) => {
// 						doc_comments.push(doc.value());
// 					},
// 					_ => {}
// 				}
// 			}
// 		}
// 	});
// 	// _handle_doc(doc_comments);
// }
