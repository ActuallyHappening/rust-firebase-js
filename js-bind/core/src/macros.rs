use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_quote, Attribute};

use crate::config::Bundle;

#[cfg(test)]
fn assert_eq_tokens(left: TokenStream, right: TokenStream) {
	assert_eq!(left.to_string(), right.to_string());
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
/// let expected = quote!{ #[wasm_bindgen] };
/// assert_eq!(attrs.to_string(), expected.to_string());
/// ```
/// 
/// ### Example with 1 bundle
/// ```rust
/// use js_bind_core::config::Bundle;
/// let bundles = vec![Bundle {
/// 	if_feature: "feature-name".to_string(),
/// 	then_js_path: "js/file/path.here".to_string(),
/// 	to_build_command: "echo 'not used'".to_string(),
/// }];
/// 
/// use quote::quote;
/// let attrs = js_bind_core::macros::gen_prelude_attrs(bundles).unwrap();
/// let expected = quote!{ #[cfg_attr(feature = "feature-name", wasm_bindgen(module = "js/file/path.here"))] #[wasm_bindgen] };
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
/// #[cfg_attr(feature = "feature-name", wasm_bindgen(module = "js/file/path.here"))]
/// #[cfg_attr(feature = "feature-name2", wasm_bindgen(module = "js/file/path.here2"))]
/// #[wasm_bindgen]
/// };
/// assert_eq!(attrs.to_string(), expected.to_string());
/// ```
pub fn gen_prelude_attrs(bundles: Vec<Bundle>) -> syn::Result<TokenStream> {
	impl Bundle {
		fn into_conditional_attr(self) -> Attribute {
			let feature_name = self.if_feature;
			let module_path = self.then_js_path;
			parse_quote! {
				#[cfg_attr(feature = #feature_name, wasm_bindgen(module = #module_path))]
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
		.chain(std::iter::once(Ok(quote! {#[wasm_bindgen]})))
		.collect()
}

#[cfg(test)]
mod prelude_tests {
	use super::*;

	#[test]
	fn test_prelude_attrs() {
		let attrs_empty = quote! { #[wasm_bindgen] };
		assert_eq_tokens(attrs_empty, gen_prelude_attrs(vec![]).unwrap());

		let attrs1 = quote! {
			#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/target/js/bundle-es.js"))]
			#[wasm_bindgen]
		};
		let bundles1 = vec![Bundle {
			if_feature: "web-not-node".to_string(),
			then_js_path: "/target/js/bundle-es.js".to_string(),
			to_build_command: "".to_string(),
		}];
		assert_eq_tokens(attrs1, gen_prelude_attrs(bundles1).unwrap());

		let attrs2 = quote! {
			#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/target/js/bundle-es.js"))]
			#[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/target/js/bundle-cjs.js"))]
			#[wasm_bindgen]
		};
		let bundles2 = vec![
			Bundle {
				if_feature: "web-not-node".to_string(),
				then_js_path: "/target/js/bundle-es.js".to_string(),
				to_build_command: "".to_string(),
			},
			Bundle {
				if_feature: "node-not-web".to_string(),
				then_js_path: "/target/js/bundle-cjs.js".to_string(),
				to_build_command: "".to_string(),
			},
		];
		assert_eq_tokens(attrs2, gen_prelude_attrs(bundles2).unwrap());
	}

	#[test]
	fn test_bundle_into_conditional_attr() {
		let test_attr: Attribute = parse_quote! {
			#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/target/js/bundle-es.js"))]
		};
		let received_attr = Bundle {
			if_feature: "web-not-node".to_string(),
			then_js_path: "/target/js/bundle-es.js".to_string(),
			to_build_command: "".to_string(),
		}
		.into_conditional_attr();
		assert_eq!(test_attr, received_attr);
	}
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Attrs {
	js_module: Option<String>,
	conditional_attrs: bool,
	inject_docs: bool,
	extract_tests: bool,
}

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
						"js_module" => {
							input.parse::<syn::Token![=]>()?;
							let js_module: syn::LitStr = input.parse()?;
							attrs.js_module = Some(js_module.value());
						}
						"conditional_attrs" => {
							attrs.conditional_attrs = true;
						}
						"inject_docs" => {
							attrs.inject_docs = true;
						}
						"extract_tests" => {
							attrs.extract_tests = true;
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
						"Expected attribute name"
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
		assert_eq!(Attrs::default(), parse_attr(quote!{}).unwrap());
		assert_eq!(Attrs {
			js_module: Some("firebase/app".to_string()),
			..Attrs::default()
		}, parse_attr(quote!{js_module = "firebase/app"}).unwrap());
		assert_eq!(Attrs {
			conditional_attrs: true,
			..Attrs::default()
		}, parse_attr(quote!{conditional_attrs}).unwrap());
		assert_eq!(Attrs {
			inject_docs: true,
			..Attrs::default()
		}, parse_attr(quote!{inject_docs}).unwrap());
		assert_eq!(Attrs {
			extract_tests: true,
			..Attrs::default()
		}, parse_attr(quote!{extract_tests}).unwrap());
		assert_eq!(Attrs {
			js_module: Some("firebase/app".to_string()),
			conditional_attrs: true,
			inject_docs: true,
			extract_tests: true,
		}, parse_attr(quote!{js_module = "firebase/app", conditional_attrs, inject_docs, extract_tests}).unwrap());
	}
}

pub fn js_bind_impl(attrs: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
	let attrs = parse_attr(attrs)?;

	let prelude = gen_prelude_attrs(vec![])?;

	Ok(quote! {
		#prelude
		#input
	})
}
