use std::unimplemented;

use proc_macro2::{Span, TokenStream};
use quote::quote;
#[allow(unused_imports)]
use quote::ToTokens;
use smart_default::SmartDefault;
use syn::{
	parse::Parse, parse_quote, spanned::Spanned, Attribute, Expr, ExprLit, ItemForeignMod, Lit, Meta,
};

use crate::config::{Bundle, Config, DocTestGen};

#[cfg(test)]
fn assert_eq_tokens(left: TokenStream, right: TokenStream) {
	// assert_eq!(left.to_string(), right.to_string(), "left: {}\nright: {}", left.to_string(), right.to_string());
	let left = left.to_string();
	let right = right.to_string();
	assert_eq!(left, right);
}

/// Generates conditional compilation attributes changing the wasm_bindgen module path,
/// depending on the feature name.
///
/// ## Examples
/// ### Example config:
/// ```toml
/// [[bundles]]
/// if = "feature-name"
/// then = "js/file/path.here"
/// to-build = "echo 'not used'"
/// ```
/// ### Example with 0 bundles
/// ```rust
/// use js_bind_core::config::Bundle;
/// let bundles = vec![];
///
/// use quote::quote;
/// let attrs = js_bind_core::macros::gen_prelude_attrs(bundles).unwrap();
/// let expected = quote!{ };
/// assert_eq!(attrs.to_string(), expected.to_string());
/// ```
///
/// ### Example with 1 bundle
/// ```rust
/// use js_bind_core::config::Bundle;
/// let bundles = vec![Bundle {
/// 	if_feature: "feature-name1".to_string(),
/// 	then_js_path: "js/file/path.here".to_string(),
/// 	to_build_command: "echo 'not used'".to_string(),
/// }];
///
/// use quote::quote;
/// let attrs = js_bind_core::macros::gen_prelude_attrs(bundles).unwrap();
/// let expected = quote!{ #[cfg_attr(feature = "feature-name1", ::wasm_bindgen::prelude::wasm_bindgen(module = "/js/file/path.here"))] };
/// assert_eq!(attrs.to_string(), expected.to_string());
/// ```
///
/// ### Example with 2 bundles
/// ```rust
/// use js_bind_core::config::Bundle;
/// let bundles = vec![
/// 	Bundle {
/// 		if_feature: "feature-name".to_string(),
/// 		then_js_path: "js/file/path.here".to_string(),
/// 		to_build_command: "echo 'not used'".to_string(),
/// 	},
/// 	Bundle {
/// 		if_feature: "feature-name2".to_string(),
/// 		then_js_path: "js/file/path.here2".to_string(),
/// 		to_build_command: "echo 'not used'".to_string(),
/// 	},
/// ];
///
/// use quote::quote;
/// let attrs = js_bind_core::macros::gen_prelude_attrs(bundles).unwrap();
/// let expected = quote!{
/// #[cfg_attr(feature = "feature-name", ::wasm_bindgen::prelude::wasm_bindgen(module = "/js/file/path.here"))]
/// #[cfg_attr(feature = "feature-name2", ::wasm_bindgen::prelude::wasm_bindgen(module = "/js/file/path.here2"))]
/// };
/// assert_eq!(attrs.to_string(), expected.to_string());
/// ```
pub fn gen_prelude_attrs(bundles: Vec<Bundle>) -> syn::Result<TokenStream> {
	impl Bundle {
		fn into_conditional_attr(self) -> Attribute {
			let feature_name = self.if_feature;
			let module_path = self.then_js_path;
			assert_ne!(feature_name, "");
			// assert first char isn't '/'
			assert_ne!(module_path.chars().next().unwrap(), '/', r##"
Paths in [[bundles]].then must be relative to package root, not absolute.
Consider removing the leading '/' from the path: "{:?}""##, module_path);
			let module_path = format!("/{}", module_path);
			parse_quote! {
				#[cfg_attr(feature = #feature_name, ::wasm_bindgen::prelude::wasm_bindgen(module = #module_path))]
			}
		}
	}

	bundles
		.into_iter()
		.map(|bundle| {
			let attr = bundle.into_conditional_attr();
			Ok(quote! {#attr})
		})
		// Adds #[wasm_bindgen] attribute as a fallback
		// .chain(std::iter::once(Ok(quote! {#[wasm_bindgen]})))
		.collect()
}

#[cfg(test)]
mod prelude_tests {
	use super::*;

	#[test]
	fn debug() {
		let input = quote! {
			extern "C" {
				/// Takes a config object and returns a firebase app instance
				#[wasm_bindgen(js_name = "initializeApp", catch)]
				pub fn initialize_app(config: JsValue) -> Result<JsValue, JsValue>;
			}
		};

		let expected = quote! {
			#[cfg_attr(feature = "web-not-node", ::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-esm.js"))]
			#[cfg_attr(feature = "node-not-web", ::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-cjs.js"))]
			extern "C" {
				/// Takes a config object and returns a firebase app instance
				#[wasm_bindgen(js_name = "initializeApp", catch)]
				pub fn initialize_app(config: JsValue) -> Result<JsValue, JsValue>;
			}
		};

		let returned = js_bind_impl(quote! {config_path = "../examples/testing-configs/firebase.js-bind.toml", conditional_attrs}, input).unwrap();

		assert_eq_tokens(expected, returned);
	}

	#[test]
	fn test_prelude_attrs() {
		let attrs_empty = quote! {};
		assert_eq_tokens(attrs_empty, gen_prelude_attrs(vec![]).unwrap());

		let attrs1 = quote! {
			#[cfg_attr(feature = "web-not-node", ::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-es.js"))]
		};
		let bundles1 = vec![Bundle {
			if_feature: "web-not-node".to_string(),
			then_js_path: "target/js/bundle-es.js".to_string(),
			to_build_command: "".to_string(),
		}];
		assert_eq_tokens(attrs1, gen_prelude_attrs(bundles1).unwrap());

		let attrs2 = quote! {
			#[cfg_attr(feature = "web-not-node", ::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-es.js"))]
			#[cfg_attr(feature = "node-not-web", ::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-cjs.js"))]
		};
		let bundles2 = vec![
			Bundle {
				if_feature: "web-not-node".to_string(),
				then_js_path: "target/js/bundle-es.js".to_string(),
				to_build_command: "".to_string(),
			},
			Bundle {
				if_feature: "node-not-web".to_string(),
				then_js_path: "target/js/bundle-cjs.js".to_string(),
				to_build_command: "".to_string(),
			},
		];
		assert_eq_tokens(attrs2, gen_prelude_attrs(bundles2).unwrap());
	}

	#[test]
	fn test_bundle_into_conditional_attr() {
		let test_attr: Attribute = parse_quote! {
			#[cfg_attr(feature = "web-not-node", ::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-es.js"))]
		};
		let received_attr: Attribute = Bundle {
			if_feature: "web-not-node".to_string(),
			then_js_path: "target/js/bundle-es.js".to_string(),
			to_build_command: "".to_string(),
		}
		.into_conditional_attr();
		// assert_eq!(test_attr.to_token_stream().to_string(), received_attr.to_token_stream().to_string());
		assert_eq_tokens(test_attr.to_token_stream(), received_attr.to_token_stream());
		assert_eq!(test_attr, received_attr);
	}
}

#[derive(Debug, SmartDefault)]
pub struct Attrs {
	#[default = "js-bind.toml"]
	config_path: String,
	js_module: Option<String>,

	fallback: bool,
	conditional_attrs: bool,
	#[default(Span::call_site())]
	conditional_attrs_span: Span,

	inject_docs: bool,

	extract_tests: bool,
	#[default(Span::call_site())]
	extract_tests_span: Span,
}

// implement PartialEq and Eq for Attrs, ignoring spans
impl PartialEq for Attrs {
	fn eq(&self, other: &Self) -> bool {
		self.config_path == other.config_path
			&& self.js_module == other.js_module
			&& self.fallback == other.fallback
			&& self.conditional_attrs == other.conditional_attrs
			&& self.inject_docs == other.inject_docs
			&& self.extract_tests == other.extract_tests
	}
}
impl Eq for Attrs {}

pub fn parse_attr(attr: TokenStream) -> syn::Result<Attrs> {
	impl Parse for Attrs {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let mut attrs = Attrs::default();

			// returns default if no attributes were passed, like `#[js_bind]`
			if input.is_empty() {
				return Ok(attrs);
			}

			while !input.is_empty() {
				let lookahead = input.lookahead1();
				if lookahead.peek(syn::Ident) {
					let ident: syn::Ident = input.parse()?;
					match ident.to_string().as_str() {
						"config_path" => {
							input.parse::<syn::Token![=]>()?;
							let config_path: syn::LitStr = input.parse()?;
							attrs.config_path = config_path.value();
						}
						"js_module" => {
							input.parse::<syn::Token![=]>()?;
							let js_module: syn::LitStr = input.parse()?;
							attrs.js_module = Some(js_module.value());
						}
						"conditional_attrs" => {
							attrs.conditional_attrs = true;
							attrs.conditional_attrs_span = ident.span();
						}
						"fallback" => {
							attrs.fallback = true;
						}
						"inject_docs" => {
							attrs.inject_docs = true;
						}
						"extract_tests" => {
							attrs.extract_tests = true;
							attrs.extract_tests_span = ident.span();
						}
						_ => {
							return Err(syn::Error::new(
								ident.span(),
								format!("Unknown attribute: {}", ident),
							))
						}
					}
				} else {
					let mut base_error = lookahead.error();
					base_error.combine(syn::Error::new(
						input.span(),
						// format!("Expected attribute name, current attr: {:?}", &attrs),
						"Expected attribute name",
					));
					return Err(base_error);
				}
				if input.peek(syn::Token![,]) {
					input.parse::<syn::Token![,]>()?;
				}
			}
			Ok(attrs)
		}
	}

	syn::parse2(attr)
}

#[cfg(test)]
mod parse_attrs_tests {
	use super::*;

	#[test]
	fn test_attrs_parse_args() {
		assert_eq!(Attrs::default(), parse_attr(quote! {}).unwrap());
		assert_eq!(
			Attrs {
				config_path: "js-bind.toml".to_string(),
				..Attrs::default()
			},
			parse_attr(quote! {config_path = "js-bind.toml"}).unwrap()
		);
		assert_eq!(
			Attrs {
				js_module: Some("firebase/app".to_string()),
				..Attrs::default()
			},
			parse_attr(quote! {js_module = "firebase/app"}).unwrap()
		);
		assert_eq!(
			Attrs {
				conditional_attrs: true,
				..Attrs::default()
			},
			parse_attr(quote! {conditional_attrs}).unwrap()
		);
		assert_eq!(
			Attrs {
				inject_docs: true,
				..Attrs::default()
			},
			parse_attr(quote! {inject_docs}).unwrap()
		);
		assert_eq!(
			Attrs {
				extract_tests: true,
				..Attrs::default()
			},
			parse_attr(quote! {extract_tests}).unwrap()
		);
		assert_eq!(Attrs {
			config_path: "js-bind.toml".into(),
			js_module: Some("firebase/app".into()),
			fallback: true,
			conditional_attrs: true,
			inject_docs: true,
			extract_tests: true,

			..Default::default()
		}, parse_attr(quote!{config_path = "js-bind.toml", js_module = "firebase/app", fallback, conditional_attrs, inject_docs, extract_tests}).unwrap());
	}
}

/// Generates test items suitable to be ran by `wasm-bindgen-test`
///
/// ```rust
/// use js_bind_core::config::DocTestGen;
/// use js_bind_core::macros::gen_doc_tests;
///
/// let toml_str = r##"
/// template = """
/// use wasm_bindgen_test::wasm_bindgen_test as test;
/// #[test]
/// fn test_generated() {
/// 	#code
/// }
/// """
/// "##;
///
/// let config: DocTestGen = toml::from_str(toml_str).expect("to parse");
/// assert_eq!(config.template.split("\n").collect::<Vec<_>>()[1], "#[test]");
///
/// let attributes = vec![
///		syn::parse_quote!{ #[doc = r#"
///		Example test:
/// 	```rust
/// 		assert_eq!(1, 1);
/// 	```
/// "#]}
/// ];
///
/// let expected = quote::quote!{
/// use wasm_bindgen_test::wasm_bindgen_test as test;
/// #[test]
/// fn test_prefix_1() {
/// 	assert_eq!(1, 1);
/// }
/// };
///
/// let generated = gen_doc_tests(&config, &attributes, "test_prefix".to_string()).unwrap();
///
/// assert_eq!(expected.to_string(), generated.to_string());
/// ```
pub fn gen_doc_tests(
	config: &DocTestGen,
	attrs: &Vec<Attribute>,
	name: String,
) -> syn::Result<TokenStream> {
	fn extract_documentation(attrs: &Vec<Attribute>) -> Vec<String> {
		attrs
			.iter()
			.filter_map(|attr| {
				if let Meta::NameValue(meta_name_value) = &attr.meta {
					if meta_name_value.path.is_ident("doc") {
						match &meta_name_value.value {
							Expr::Lit(ExprLit {
								lit: Lit::Str(doc), ..
							}) => {
								return Some(doc.value());
							}
							_ => None,
						}
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect()
	}

	struct TestableCodeBlock {
		/// Lines of code to be subbed in for #code var
		code: Vec<String>,
		/// Full name of test
		name: String,
		// flags: Vec<Flag>,
	}

	fn parse_documentation(lines: Vec<String>) -> Vec<TestableCodeBlock> {
		unimplemented!()
	}
	unimplemented!()
}

pub fn js_bind_impl(attr: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
	let attr_span = attr.span();
	let attrs = parse_attr(attr)?;
	let input_extern: ItemForeignMod = syn::parse2(input.clone())?;

	let config = Config::from_cwd(&attrs.config_path).expect("Cannot parse config");

	let mut fallback = TokenStream::new();
	if attrs.fallback {
		fallback = quote! {
			#[::wasm_bindgen::prelude::wasm_bindgen]
		};
	}

	let mut prelude = TokenStream::new();
	if attrs.conditional_attrs {
		let err_msg = "Expected config to have a [[bundles]] table because #[js_bind(conditional_attrs)] was specified which requires [[bundles]].if and [[bundles]].then to have at least one entry";
		let bundles = config
			.bundles
			.ok_or_else(|| syn::Error::new(attrs.conditional_attrs_span, err_msg))?;
		prelude = gen_prelude_attrs(bundles)?;
	}

	let mut doc_test_gen = TokenStream::new();
	if attrs.extract_tests {
		let err_msg = "Expected config to have a [doctestgen] table because #[js_bind(extract_tests)] was specified which requires [doctestgen.template] to be specified";
		let config = config
			.doc_test_gen
			.ok_or_else(|| syn::Error::new(attrs.extract_tests_span, err_msg))?;
		// TODO: Loop through all items/fns and generate tests for them, maybe extract into seperate func?
		// doc_test_gen = gen_doc_tests(&config, &attributes)?;
	}

	let processed_output = input;

	Ok(quote! {
		#prelude
		#fallback
		#processed_output

		#doc_test_gen
	})
}
