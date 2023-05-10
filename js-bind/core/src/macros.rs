use proc_macro2::TokenStream;

mod prelude_attrs {
	use super::*;

	
}

pub fn js_bind_impl(attrs: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
	Ok(input)
}