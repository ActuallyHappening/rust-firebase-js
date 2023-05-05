use proc_macro2::TokenStream;
use quote::*;
use serde::{Deserialize, Serialize};
use syn::parse::*;
use syn::*;

use crate::config::Bundles;

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

/// Takes valid bundles and generates code that conditionally compiles the specified `then` clause,
/// i.e. a path to a JS file.
fn gen_bundle_prelude(config: &Vec<Bundles>) -> TokenStream {
	fn gen_wasm_bindgen_attr(feature_predicate: &str, js_module: &str) -> TokenStream {
		quote! {
			#[cfg_attr(#feature_predicate, wasm_bindgen(module = #js_module))]
		}
	}

	/// Gives warning if no features are enabled,
	/// more than one feature is enabled,
	/// or all features are enabled
	fn gen_feature_restriction_prelude(config: &Vec<Bundles>) -> TokenStream {
		let mut expanded = TokenStream::new();
		let features: Vec<_> = config.into_iter().map(|b| &b.if_feature).collect();

		// no features
		expanded.extend(quote! {
			#[cfg(not(any(#(#features),*)))]
			compile_error!("No features enabled!");
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


	let mut expanded = TokenStream::new();

	

	expanded
}

pub fn _js_bind_impl(
	attrs: proc_macro2::TokenStream,
	input: proc_macro2::TokenStream,
) -> std::result::Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
	let attrs = syn::parse2::<Attrs>(attrs).map_err(Error::into_compile_error)?;

	let expanded = quote! {

	};

	Ok(expanded)
}
