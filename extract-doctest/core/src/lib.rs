#![allow(unused_imports)]

use std::unimplemented;

use proc_macro2::TokenStream;
use serde::Deserialize;
use syn::{Attribute, Expr, ExprLit, ItemFn, ItemUse, Lit, Meta, Item, spanned::Spanned};
use quote::quote;

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

pub fn raw_into_processable_documentations(raw_input: TokenStream) -> syn::Result<Vec<Vec<Attribute>>> {
	// Parse input into syn::Item
	let input_span = raw_input.span();
	match syn::parse2::<Item>(raw_input) {
		Ok(item) => {
			Ok(vec![])
		},
		Err(e) => {
			let mut base_err = syn::Error::new(input_span, "#[extract_docs] Failed to parse input as a rust item. \
				Make sure you are using this macro on a valid function, struct or extern block.");
			base_err.combine(e);
			return Err(base_err);
		}
	}
}

pub fn extract_doctests(raw_input: TokenStream) -> syn::Result<TokenStream> {
	raw_into_processable_documentations(raw_input)?;

	Ok(quote!{})
}

pub fn extract_doctest_impl(
	raw_attrs: TokenStream,
	raw_input: TokenStream,
) -> syn::Result<TokenStream> {
	// make sure raw_attrs is empty
	if !raw_attrs.is_empty() {
		return Err(syn::Error::new_spanned(
			raw_attrs,
			"extract_doctest_impl does not take any attributes (yet)",
		));
	}

	let tests = extract_doctests(raw_input.clone())?;

	let expanded: TokenStream = quote! {
		#raw_input

		#tests
	};

	return Ok(expanded);
}
