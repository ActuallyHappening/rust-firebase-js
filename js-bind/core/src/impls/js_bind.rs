use quote::*;
use serde::{Deserialize, Serialize};
use syn::parse::*;
use syn::*;

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

pub fn _js_bind_impl(
	attrs: proc_macro2::TokenStream,
	input: proc_macro2::TokenStream,
) -> std::result::Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
	let attrs = syn::parse2::<Attrs>(attrs).map_err(Error::into_compile_error)?;

	let expanded = quote! {

	};

	Ok(expanded)
}
