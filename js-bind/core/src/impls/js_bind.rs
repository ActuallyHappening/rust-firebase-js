use proc_macro2::{Span, TokenStream};
use quote::*;
use serde::{Deserialize, Serialize};
use std::result::Result;
use syn::parse::*;
use syn::*;

use crate::config::{Bundle, Config, FromTOMLCwd};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attrs {
	js_module: String,
	docs: bool,
	tests: bool,
}

impl Parse for Attrs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let name = input.parse::<Ident>()?;
		if name.to_string() != "js_module" {
			Err(Error::new(
				name.span(),
				"Expected `js_module = \"...\"` at the start of your `#[js_bind(js_module = ...)]`",
			))?
		}
		input.parse::<Token![=]>()?;
		let js_module = input.parse::<LitStr>()?.value();

		// loop until empty, try to parse optional docs = true and tests = true
		let mut docs = false;
		let mut tests = false;
		while !input.is_empty() {
			input.parse::<Token![,]>()?;
			let name = input.parse::<Ident>()?;
			match name.to_string().as_str() {
				"docs" => {
					input.parse::<Token![=]>()?;
					docs = input.parse::<LitBool>()?.value();
				}
				"tests" => {
					input.parse::<Token![=]>()?;
					tests = input.parse::<LitBool>()?.value();
				}
				_ => Err(Error::new(
					name.span(),
					format!(
						r##"Unknown option: "{}"; Expected either "docs" or "tests"
							Have you forgotten to add `#[js_bind(js_module = "...")]` at the top level of your `extern "C" {{ ... }}` block?
							"##,
						name.to_string()
					),
				))?,
			}
			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}

		Ok(Self {
			js_module,
			tests,
			docs,
		})
	}
}

mod prelude {
	use super::*;

	/// Generates a conditional wasm_bindgen attribute to comple in `module = "foobar"`
	fn gen_wasm_bindgen_attr(feature_predicate: &str, js_file_path: &str) -> TokenStream {
		assert_ne!(js_file_path.chars().nth(0).unwrap(), '/', "js_file_path must be relative to the project root, not the filesystem root. The implied '/' is added");
		let js_file_path = format!("/{}", js_file_path);
		return quote! {
			#[cfg_attr(feature = #feature_predicate, wasm_bindgen(module = #js_file_path))]
		};
	}

	/// Gives warning if no features are enabled,
	/// TODO: more than one feature is enabled,
	/// or all features are enabled
	fn gen_feature_restriction_prelude(config: &Vec<Bundle>) -> TokenStream {
		let mut expanded = TokenStream::new();
		let features: Vec<_> = config.into_iter().map(|b| &b.if_feature).collect();

		let msg: String = format!(
			"No features enabled! Consider enabling one of these features: {:?}",
			features
		);

		// no features
		expanded.extend(quote! {
			#[cfg(not(any(#(feature = #features),*)))]
			compile_error!(#msg);
		});

		// more than one feature
		// expanded.extend(quote! {
		// 	#[cfg(any(#(#features),*)) && not(any(#(#features),*))]
		// 	compile_error!("More than one feature enabled!");
		// });

		// All features
		// expanded.extend(quote! {
		// 	#[cfg(all(#(#features),*))]
		// 	eprintln!("All features enabled!");
		// });

		expanded
	}

	/// Generates all of the conditional compilation attributes
	fn gen_bindgen_attrs(config: &Vec<Bundle>) -> TokenStream {
		let mut expanded = TokenStream::new();
		config.into_iter().for_each(|bundle| {
			expanded.extend(gen_wasm_bindgen_attr(
				&bundle.if_feature,
				&bundle.then_js_path,
			));
		});
		expanded
	}

	/// Takes valid bundles and generates code that conditionally compiles the specified `then` clause,
	/// i.e. a path to a JS file.
	pub fn gen_bundle_prelude(config: &Vec<Bundle>) -> TokenStream {
		if config.len() == 0 {
			return Error::new(
				Span::call_site(),
				"Config does not contain any bundles. Add a bundle to conditionally compile.",
			)
			.into_compile_error();
		}

		let mut expanded = TokenStream::new();

		expanded.extend(gen_feature_restriction_prelude(config));
		expanded.extend(gen_bindgen_attrs(config));

		expanded
	}

	#[cfg(test)]
	mod test_gen_prlude {
		use super::*;
		#[test]
		fn test_gen_wasm_bindgen_attr() {
			let attr: TokenStream = gen_wasm_bindgen_attr("compile-node-pls", "my/js-module yes");

			let str_repr1 = attr.to_string().replace(" ", "");
			let str_repr2 = quote! {#attr}.to_string().replace(" ", "");
			let expected =
				r##"#[cfg_attr(feature = "compile-node-pls", wasm_bindgen(module = "/my/js-module yes"))]"##
					.replace(" ", "");

			assert_eq!(str_repr1, expected);
			assert_eq!(str_repr2, expected);
		}

		#[test]
		fn test_gen_feature_restriction_prelude_generic() {
			let config: Vec<Bundle> = vec![Bundle {
				if_feature: "compile-web-pls".into(),
				then_js_path: "maybe-js/foobar.js".into(),
				to_build_command: "ignored".into(),
			}];

			let generated = gen_feature_restriction_prelude(&config);

			let str_reprs = vec![generated.to_string(), quote! {#generated}.to_string()];
			str_reprs
				.into_iter()
				// .map(|r| r.replace(" ", ""))
				.for_each(|repr| {
					// println!("Repr: {:?}", repr);
					assert!(repr.contains(r#"No features enabled"#));
					assert!(repr.contains("compile_error"));
				});
		}

		#[test]
		fn test_gen_feature_restriction_prelude_specific1() {
			let config = vec![Bundle {
				if_feature: "compile-web-pls".into(),
				then_js_path: "maybe-js/foobar.js".into(),
				to_build_command: "ignored".into(),
			}];

			let generated = gen_feature_restriction_prelude(&config);

			let expected = quote! {
				#[cfg(not(any(feature = "compile-web-pls")))]
				compile_error!("No features enabled! Consider enabling one of these features: [\"compile-web-pls\"]");
			};

			assert_eq!(generated.to_string(), expected.to_string());
		}

		#[test]
		fn test_bindgen_attrs_generic() {
			let config = vec![Bundle {
				if_feature: "compile-web-pls".into(),
				then_js_path: "maybe-js/foobar.js".into(),
				to_build_command: "ignored".into(),
			}];

			let generated = gen_bindgen_attrs(&config);

			let str_reprs = vec![generated.to_string(), quote! {#generated}.to_string()];
			str_reprs
			.into_iter()
			.map(|r| r.replace(" ", ""))
			.for_each(|repr| {
				// println!("Repr: {:?}", repr);
				assert!(repr.contains(&r#"#[cfg_attr(feature = "compile-web-pls", wasm_bindgen(module = "/maybe-js/foobar.js"))]"#.replace(" ", "")));
			});
		}

		#[test]
		fn test_bindgen_attrs_specific1() {
			let config = vec![Bundle {
				if_feature: "compile-web-pls".into(),
				then_js_path: "maybe-js/foobar.js".into(),
				to_build_command: "ignored".into(),
			}];

			let generated = gen_bindgen_attrs(&config);

			let expected = quote! {
				#[cfg_attr(feature = "compile-web-pls", wasm_bindgen(module = "/maybe-js/foobar.js"))]
			};

			assert_eq!(generated.to_string(), expected.to_string());
		}

		#[test]
		fn test_bindgen_attrs_specific2() {
			let config = vec![
				Bundle {
					if_feature: "compile-web-pls".into(),
					then_js_path: "maybe-js/foobar.js".into(),
					to_build_command: "ignored".into(),
				},
				Bundle {
					if_feature: "compile-node-pls".into(),
					then_js_path: "anything/baz.js".into(),
					to_build_command: "ignoredagain".into(),
				},
			];

			let generated = gen_bindgen_attrs(&config);

			let expected = quote! {
				#[cfg_attr(feature = "compile-web-pls", wasm_bindgen(module = "/maybe-js/foobar.js"))]
				#[cfg_attr(feature = "compile-node-pls", wasm_bindgen(module = "/anything/baz.js"))]
			};

			assert_eq!(generated.to_string(), expected.to_string());
		}

		#[test]
		fn test_gen_bundle_prelude_specific1() {
			let config = vec![Bundle {
				if_feature: "compile-web-pls".into(),
				then_js_path: "maybe-js/foobar.js".into(),
				to_build_command: "ignored".into(),
			}];

			let generated = gen_bundle_prelude(&config);

			let expected = quote! {
				#[cfg(not(any(feature = "compile-web-pls")))]
				compile_error!("No features enabled! Consider enabling one of these features: [\"compile-web-pls\"]");
				#[cfg_attr(feature = "compile-web-pls", wasm_bindgen(module = "/maybe-js/foobar.js"))]
			};

			assert_eq!(generated.to_string(), expected.to_string());
		}

		#[test]
		fn test_gen_bundle_prelude_specific2() {
			let config = vec![
				Bundle {
					if_feature: "compile-web-pls".into(),
					then_js_path: "maybe-js/foobar.js".into(),
					to_build_command: "ignored".into(),
				},
				Bundle {
					if_feature: "compile-node-pls".into(),
					then_js_path: "anything/baz.js".into(),
					to_build_command: "ignoredagain".into(),
				},
			];

			let generated = gen_bundle_prelude(&config);

			let expected = quote! {
				#[cfg(not(any(feature = "compile-web-pls", feature = "compile-node-pls")))]
				compile_error!("No features enabled! Consider enabling one of these features: [\"compile-web-pls\", \"compile-node-pls\"]");
				#[cfg_attr(feature = "compile-web-pls", wasm_bindgen(module = "/maybe-js/foobar.js"))]
				#[cfg_attr(feature = "compile-node-pls", wasm_bindgen(module = "/anything/baz.js"))]
			};

			assert_eq!(generated.to_string(), expected.to_string());
		}
	}
}

mod input {
	use super::*;
	use crate::{
		config::{LockTemplate, Match, Matches, Template, Templates},
		docs::Docs,
	};
	use derive_new::new;
	use std::{str::FromStr, path::PathBuf};
	use syn::visit_mut::VisitMut;

	#[derive(new)]
	struct DocumentationMutVistor<'config> {
		pub templates: &'config Templates,
		pub attrs: &'config Attrs,
	}

	impl<'config> VisitMut for DocumentationMutVistor<'_> {
		fn visit_foreign_item_fn_mut(&mut self, func: &mut ForeignItemFn) {
			self.handle_fn(func);

			// continues recursive search, not really required but eh
			// visit_mut::visit_foreign_item_fn_mut(self, i);
		}
	}

	impl<'config> DocumentationMutVistor<'config> {
		// fn new(templates: &'config Templates) -> Self {
		// 	Self { templates }
		// }

		/// Must mutate the function to add docs as desired
		/// Uses templates to add correct docs
		fn handle_fn(&mut self, func: &mut ForeignItemFn) {
			let func_name = func.sig.ident.to_string();
			let templates = self.templates;

			let signature = match WasmBindgenOptions::get_from_attrs_or_empty(&func.attrs) {
				Ok(options) => options,
				Err(e) => {
					panic!("Error parsing wasm_bindgen options: {:?}", e);
				}
			};
			// find match
			let template = templates.find_matching_template(&signature);
			match template {
				Some(template) => {
					println!(
						"Found matching template {:?} for func name '{}'",
						template, &func_name
					);

					process_func(func, template, &self.attrs);
				}
				None => {
					println!(
						"No templates matched for wasmbindgen signature {:?} on func name'{}'",
						signature, &func_name
					);
				}
			}
		}
	}

	/// Represents the options passed to `#[wasm_bindgen]` procmacro.
	/// An empty [options] field means no options were passed, OR no attribute was found.
	#[derive(Debug, PartialEq)]
	struct WasmBindgenOptions {
		pub options: Vec<WasmBindgenOption>,
	}

	impl Parse for WasmBindgenOptions {
		/// Parses the arguments of the `wasm_bindgen` attribute
		/// e.g. `#[wasm_bindgen(catch)]`
		fn parse(content: ParseStream) -> Result<Self, Error> {
			let mut options = Vec::new();
			let option = content.parse::<Ident>()?;
			let option = WasmBindgenOption::from_str(&option.to_string()).map_err(|_| {
				Error::new(
					option.span(),
					format!("Unknown option `{}`", option.to_string()),
				)
			})?;
			options.push(option);
			if !content.is_empty() {
				content.parse::<Token![,]>()?;
			}
			Ok(Self { options })
		}
	}

	impl WasmBindgenOptions {
		/// Tries to parse the first `wasm_bindgen` attribute from a list of attributes
		///
		/// Returns [None] if none found
		///
		/// Common use cases involve parsing the attributes of a function, because this ignores non-wasm_bindgen attributes.
		/// Combines options, as this is the behaviour of wasm_bindgen
		fn get_from_attrs(attrs: &Vec<Attribute>) -> Option<Result<Self, syn::Error>> {
			let mut found_wasm_bindgen = false;
			let mut options = Vec::new();
			for attr in attrs {
				println!("attr: {:?}", attr);
				let parsed: WasmBindgenOptions = match Self::parse_from_attr(attr) {
					Some(parsed) => {
						// Is wasm-bindgen attr
						match parsed {
							Ok(parsed) => parsed,
							Err(e) => return Some(Err(e)),
						}
					}
					None => continue, // Not a wasm_bindgen attribute
				};
				found_wasm_bindgen = true;
				options.extend(parsed.options);
			}
			if !found_wasm_bindgen {
				None
			} else {
				Some(Ok(Self { options }))
			}
		}

		/// Tries to parse for `wasm_bindgen` attrs, or returns an empty [Self] if none found
		fn get_from_attrs_or_empty(attrs: &Vec<Attribute>) -> Result<Self, syn::Error> {
			Self::get_from_attrs(attrs).unwrap_or(Ok(Self { options: vec![] }))
		}

		/// Parses a single attribute as a wasm_bindgen attribute
		/// Returns [None] if the attribute is not wasm_bindgen
		fn parse_from_attr(attr: &Attribute) -> Option<Result<WasmBindgenOptions, syn::Error>> {
			// check attr is actually wasm_bindgen
			let path = &attr.path();
			let ident = path.get_ident().expect("attr to have ident");
			if ident.to_string() != "wasm_bindgen" {
				return None;
			}

			// Check if no args present
			if let Meta::Path(_) = attr.meta {
				return Some(Ok(WasmBindgenOptions { options: vec![] }));
			}

			let args = match attr
				.parse_args()
				.map_err(|e| Error::new(e.span(), e.to_string()))
			{
				Ok(args) => args,
				Err(e) => return Some(Err(e)),
			};

			Some(Ok(args))
		}
	}

	impl Match {
		/// Checks if the wasm-bindgen attribute options [WasmBindgenOptions] match what the config file wants
		fn does_options_match(&self, options: &WasmBindgenOptions) -> bool {
			let options = &options.options;

			if options.is_empty() && self.empty.unwrap_or(false) {
				return true;
			}

			false
		}
	}

	impl Matches {
		fn does_options_match(&self, options: &WasmBindgenOptions) -> bool {
			self.matches.iter().any(|m| m.does_options_match(&options))
		}
	}

	impl Template {
		fn does_template_match(&self, options: &WasmBindgenOptions) -> Option<&Self> {
			if self.matches_signature.does_options_match(&options) {
				Some(self)
			} else {
				None
			}
		}
	}

	impl Templates {
		/// Tries to find a matching template given a concrete [WasmBindgenOptions], of [None] if not found
		fn find_matching_template(&self, options: &WasmBindgenOptions) -> Option<&Template> {
			self
				.templates
				.iter()
				.find_map(|t| t.does_template_match(&options))
		}
	}

	#[derive(Debug, PartialEq)]
	enum WasmBindgenOption {
		Catch,
	}

	impl FromStr for WasmBindgenOption {
		type Err = ();

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"catch" => Ok(Self::Catch),
				_ => Err(()),
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use crate::config::CodeGen;

		use super::*;

		#[test]
		fn test_documentation_tests_expand() {
			let docs = 
r##"
```rust
// JSBIND-TEST=test/subdir/example
this_should_error();
```
"##;
			let docs = Docs::new(vec![parse_quote!{
				#[doc = #docs]
			}]);
			let temp_dir = tempfile::tempdir().expect("to create temp dir");
			let dir_path = temp_dir.path().to_path_buf();
			let lock_template = LockTemplate {
				template_name_ref: "NA".into(),
				var_codegen_template: "NA".into(),
				var_documentation_template: "NA".into(),
				var_module: "foo/bar".into(),
				var_name: "example_func".into(),
			};

			process_func_tests(&docs, &lock_template, &dir_path);
		}

		/// Tests if matching documentation template expands at all
		#[test]
		fn test_process_documentation_comments_specific1() {
			let config = Config::new(
				vec![],
				CodeGen {
					output: "NA".into(),
					templates: Templates {
						templates: vec![Template {
							matches_signature: Matches {
								matches: vec![Match {
									empty: Some(true),
									..Default::default()
								}],
							},
							documentation_template: r##"Hello world!"##.into(),
							..Default::default()
						}],
					},
				},
				"NA".into(),
			);
			let input: ItemForeignMod = parse_quote!(
				extern "C" {
					fn foo();
				}
			);
			let attrs = Attrs {
				js_module: "test/app".into(),
				tests: false,
				docs: true,
			};

			let output = process_js_bind_input(&input, &config, &attrs).expect("to work");

			let expected_output: ItemForeignMod = parse_quote!(
				extern "C" {
					#[doc = "Hello world!"]
					fn foo();
				}
			);

			assert_eq!(
				quote! {#output}.to_string(),
				quote! {#expected_output}.to_string()
			);

			assert_eq!(output, expected_output);
		}

		/// Tests if matching documentation template expands with correct variables
		#[test]
		fn test_process_documentation_comments_specific2() {
			let config = Config::new(
				vec![],
				CodeGen {
					output: "NA".into(),
					templates: Templates {
						templates: vec![Template {
							matches_signature: Matches {
								matches: vec![Match {
									empty: Some(true),
									..Default::default()
								}],
							},
							documentation_template: r##"Function name: '#name', from js mod: '#mod'."##.into(),
							..Default::default()
						}],
					},
				},
				"NA".into(),
			);
			let input: ItemForeignMod = parse_quote!(
				extern "C" {
					fn foo();
				}
			);
			let attrs = Attrs {
				js_module: "test/app".into(),
				tests: false,
				docs: true,
			};

			let output = process_js_bind_input(&input, &config, &attrs).expect("to work");

			let expected_output: ItemForeignMod = parse_quote!(
				extern "C" {
					#[doc = "Function name: 'foo', from js mod: 'test/app'."]
					fn foo();
				}
			);

			assert_eq!(
				quote! {#output}.to_string(),
				quote! {#expected_output}.to_string()
			);

			assert_eq!(output, expected_output);
		}

		/// Tests if matching documentation template DOESN'T expand if test = false
		#[test]
		fn test_process_documentation_comments_specific3() {
			let config = Config::new(
				vec![],
				CodeGen {
					output: "NA".into(),
					templates: Templates {
						templates: vec![Template {
							matches_signature: Matches {
								matches: vec![Match {
									empty: Some(true),
									..Default::default()
								}],
							},
							documentation_template: r##"Function name: '#name', from js mod: '#mod'."##.into(),
							..Default::default()
						}],
					},
				},
				"NA".into(),
			);
			let input: ItemForeignMod = parse_quote!(
				extern "C" {
					fn foo();
				}
			);
			let attrs = Attrs {
				js_module: "test/app".into(),
				tests: false,
				docs: false,
			};

			let output = process_js_bind_input(&input, &config, &attrs).expect("to work");

			let expected_output: ItemForeignMod = parse_quote!(
				extern "C" {
					fn foo();
				}
			);

			assert_eq!(
				quote! {#output}.to_string(),
				quote! {#expected_output}.to_string()
			);

			assert_eq!(output, expected_output);
		}

		#[test]
		fn test_wasmbindgen_parse_from_attrs_specific1() {
			let func: ItemFn = parse_quote! {
				/// Some documentation
				/// more
				#[wasm_bindgen]
				fn foo() {}
			};
			let attrs = func.attrs;

			let parsed = WasmBindgenOptions::get_from_attrs(&attrs);

			let parsed = parsed.expect("parsed to be ok").expect("no errors");
			assert_eq!(parsed.options.len(), 0);
		}

		#[test]
		fn test_wasmbindgen_parse_from_attrs_specific2() {
			let func: ItemFn = parse_quote! {
				/// Some documentation
				/// more
				#[wasm_bindgen(catch)]
				fn foo() {}
			};
			let attrs = func.attrs;

			let parsed = WasmBindgenOptions::get_from_attrs(&attrs);

			let parsed = parsed.expect("parsed to be ok").expect("no errors");
			assert_eq!(parsed.options.len(), 1);
		}

		#[test]
		fn test_wasmbindgen_parse_from_attr_specific1() {
			let attr: Attribute = parse_quote! {
				#[wasm_bindgen]
			};

			let parsed = WasmBindgenOptions::parse_from_attr(&attr);

			let parsed = parsed.expect("parsed to be ok").expect("no errors");
			assert_eq!(parsed.options.len(), 0);
		}

		#[test]
		fn test_wasmbindgen_parse_from_attr_specific2() {
			let attr: Attribute = parse_quote! {
				#[wasm_bindgen(catch)]
			};

			let parsed = WasmBindgenOptions::parse_from_attr(&attr);

			let parsed = parsed.expect("parsed to be ok").expect("no errors");
			assert_eq!(parsed.options.len(), 1);
			assert_eq!(parsed.options.first().unwrap(), &WasmBindgenOption::Catch);
		}
	}

	/// Does the actualy mutation of a function, given a matching template
	fn process_func(func: &mut ForeignItemFn, matching_template: &Template, attrs: &Attrs) {
		let template = matching_template.clone();

		let func_name = func.sig.ident.to_string();
		let var_name = template.resolve_js_name(&func_name);

		let lock_template =
			LockTemplate::new_from_template(template, var_name, attrs.js_module.clone());

		if attrs.docs {
			process_func_docs(func, &lock_template);
		}
	}

	/// Adds the desired documentation using a [LockTemplate] onto the function
	fn process_func_docs(func: &mut ForeignItemFn, lock_template: &LockTemplate) {
		// Append new docs
		let new_docs = vec![lock_template.expand_documentation_template()];
		Docs::append_strings_over(new_docs, &mut func.attrs);
	}

	/// Parses the given docs into [CodeBlock]s and outputs them (using the [LockTemplate]) to the output directory
	fn process_func_tests(docs: &Docs, lock_template: &LockTemplate, output_dir: &PathBuf) {
		
	}

	/// Takes the input of the `js_bind` macro and mutates the documentation comments (according to config)
	pub fn process_js_bind_input(
		input: &ItemForeignMod,
		config: &Config,
		attrs: &Attrs,
	) -> syn::Result<ItemForeignMod> {
		let mut mutable_input = input.clone();

		let mut visitor = DocumentationMutVistor::new(&config.codegen.templates, attrs);
		visitor.visit_item_foreign_mod_mut(&mut mutable_input);

		Ok(mutable_input)
	}
}

pub fn _js_bind_impl(
	attrs: proc_macro2::TokenStream,
	input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
	let attrs = syn::parse2::<Attrs>(attrs).map_err(Error::into_compile_error)?;
	let input = syn::parse2::<ItemForeignMod>(input).map_err(Error::into_compile_error)?;
	let config = Config::from_cwd().map_err(|e| {
		Error::new(
			Span::call_site(),
			format!("Couldn't parse config: {:#?}", e),
		)
		.into_compile_error()
	})?;

	let prelude = prelude::gen_bundle_prelude(&config.bundles);
	let mutated_input =
		input::process_js_bind_input(&input, &config, &attrs).map_err(Error::into_compile_error)?;

	let expanded = quote! {
		#prelude
		// fn nothing() {};
		// #input
		#mutated_input
	};

	Ok(expanded)
}
