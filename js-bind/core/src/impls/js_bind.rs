use proc_macro2::{Span, TokenStream};
use quote::*;
use serde::{Deserialize, Serialize};
use syn::parse::*;
use syn::*;

use crate::config::{Bundles, Config, FromTOMLCwd};

#[derive(Serialize, Deserialize, Debug)]
struct Attrs {
	js_module: String,
}

impl Parse for Attrs {
	fn parse(input: ParseStream) -> Result<Self> {
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

/// Generates a conditional wasm_bindgen attribute to comple in `module = "foobar"`
fn gen_wasm_bindgen_attr(feature_predicate: &str, js_module: &str) -> TokenStream {
	return quote! {
		#[cfg_attr(feature = #feature_predicate, wasm_bindgen(module = #js_module))]
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
		#[cfg(not(any(#(#features),*)))]
		compile_error!(#msg);
	});

	// more than one feature
	// expanded.extend(quote! {
	// 	#[cfg(any(#(#features),*)) && not(any(#(#features),*))]
	// 	compile_error!("More than one feature enabled!");
	// });

	// All features
	expanded.extend(quote! {
		#[cfg(all(#(#features),*))]
		eprintln!("All features enabled!");
	});

	expanded
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_gen_wasm_bindgen_attr() {
		let attr: TokenStream = gen_wasm_bindgen_attr("compile-node-pls", "my/js-module yes");

		let str_repr1 = attr.to_string().replace(" ", "");
		let str_repr2 = quote! {#attr}.to_string().replace(" ", "");
		let expected =
			r##"#[cfg_attr(feature = "compile-node-pls", wasm_bindgen(module = "my/js-module yes"))]"##
				.replace(" ", "");

		assert_eq!(str_repr1, expected);
		assert_eq!(str_repr2, expected);
	}
}

/// Takes valid bundles and generates code that conditionally compiles the specified `then` clause,
/// i.e. a path to a JS file.
fn gen_bundle_prelude(config: &Vec<Bundles>) -> TokenStream {
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

	let mut expanded = TokenStream::new();

	expanded.extend(gen_feature_restriction_prelude(config));
	expanded.extend(gen_bindgen_attrs(config));

	expanded
}

pub fn _js_bind_impl(
	attrs: proc_macro2::TokenStream,
	input: proc_macro2::TokenStream,
) -> std::result::Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
	let attrs = syn::parse2::<Attrs>(attrs).map_err(Error::into_compile_error)?;
	let config = Config::from_cwd().map_err(|e| {
		Error::new(
			Span::call_site(),
			format!("Couldn't parse config: {:#?}", e),
		)
		.into_compile_error()
	})?;

	let prelude = gen_bundle_prelude(&config.bundles);

	let expanded = quote! {
		#prelude
		#input
	};

	Ok(expanded)
}
