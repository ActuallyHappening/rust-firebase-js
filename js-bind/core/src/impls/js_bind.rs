use proc_macro2::{Span, TokenStream};
use quote::*;
use serde::{Deserialize, Serialize};
use std::result::Result;
use syn::parse::*;
use syn::*;

use crate::{
	config::{Bundles, Config, FromTOMLCwd},
	docs::CodeBlock,
};

#[derive(Serialize, Deserialize, Debug)]
struct Attrs {
	js_module: String,
}

impl Parse for Attrs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let name = input.parse::<Ident>()?;
		if name.to_string() != "js_module" {
			Err(Error::new(name.span(), "Expected `js_module = \"...\"`"))?
		}
		input.parse::<Token![=]>()?;
		Ok(Self {
			js_module: input.parse::<LitStr>()?.value(),
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
	fn gen_feature_restriction_prelude(config: &Vec<Bundles>) -> TokenStream {
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
	fn gen_bindgen_attrs(config: &Vec<Bundles>) -> TokenStream {
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
	pub fn gen_bundle_prelude(config: &Vec<Bundles>) -> TokenStream {
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
			let config: Vec<Bundles> = vec![Bundles {
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
			let config = vec![Bundles {
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
			let config = vec![Bundles {
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
			let config = vec![Bundles {
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
				Bundles {
					if_feature: "compile-web-pls".into(),
					then_js_path: "maybe-js/foobar.js".into(),
					to_build_command: "ignored".into(),
				},
				Bundles {
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
			let config = vec![Bundles {
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
				Bundles {
					if_feature: "compile-web-pls".into(),
					then_js_path: "maybe-js/foobar.js".into(),
					to_build_command: "ignored".into(),
				},
				Bundles {
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
	use crate::{config::Template, docs::Docs};
	use std::str::FromStr;
	use syn::visit_mut::VisitMut;

	struct DocumentationMutVistor<'config> {
		pub templates: &'config Vec<Template>,
	}

	impl<'config> VisitMut for DocumentationMutVistor<'_> {
		fn visit_foreign_item_fn_mut(&mut self, func: &mut ForeignItemFn) {
			self.handle_fn(func);

			// continues recursive search, not really required but eh
			// visit_mut::visit_foreign_item_fn_mut(self, i);
		}
	}

	impl<'config> DocumentationMutVistor<'config> {
		fn new(templates: &'config Vec<Template>) -> Self {
			Self { templates }
		}

		// Must mutate the function to add docs as desired
		fn handle_fn(&mut self, func: &mut ForeignItemFn) {
			println!("Found foreign item fn: {:?}", func);

			let docs = Docs::new(func.attrs.clone());
			// let new_docs = docs.append_lines(vec!["Hello".to_owned(), "World".to_owned()]);
			// new_docs.overwrite_over(&mut func.attrs);
		}
	}

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
		fn parse_from_attr(attr: &Attribute) -> Result<WasmBindgenOptions, TokenStream> {
			// check attr is actually wasm_bindgen
			let path = &attr.path();
			let ident = path.get_ident().expect("attr to have ident");
			if ident.to_string() != "wasm_bindgen" {
				return Err(
					Error::new(
						ident.span(),
						format!(
							"Expected `wasm_bindgen` attribute, found `{}`",
							ident.to_string()
						),
					)
					.into_compile_error(),
				);
			}

			// Check if no args present
			if let Meta::Path(_) = attr.meta {
				return Ok(WasmBindgenOptions { options: vec![] });
			}

			let args = attr
				.parse_args()
				.map_err(|e| Error::new(e.span(), e.to_string()).into_compile_error())?;

			Ok(args)
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
		use super::*;

		#[test]
		fn test_wasmbindgen_parse_from_attr_specific1() {
			let attr: Attribute = parse_quote! {
				#[wasm_bindgen]
			};

			let parsed = WasmBindgenOptions::parse_from_attr(&attr);


			let parsed = parsed.expect("parsed to be ok");
			assert_eq!(parsed.options.len(), 0);
		}

		#[test]
		fn test_wasmbindgen_parse_from_attr_specific2() {
			let attr: Attribute = parse_quote! {
				#[wasm_bindgen(catch)]
			};

			let parsed = WasmBindgenOptions::parse_from_attr(&attr);

			let parsed = parsed.expect("parsed to be ok");
			assert_eq!(parsed.options.len(), 1);
			assert_eq!(parsed.options.first().unwrap(), &WasmBindgenOption::Catch);
		}
	}

	/// Takes the input of the `js_bind` macro and mutates the documentation comments (according to config)
	pub fn process_js_bind_input(
		input: &ItemForeignMod,
		config: &Config,
	) -> Result<ItemForeignMod, TokenStream> {
		let mut mutable_input = input.clone();

		let mut visitor = DocumentationMutVistor::new(&config.codegen.templates);
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
	let mutated_input = input::process_js_bind_input(&input, &config)?;

	let expanded = quote! {
		#prelude
		// fn nothing() {};
		// #input
		#mutated_input
	};

	Ok(expanded)
}
