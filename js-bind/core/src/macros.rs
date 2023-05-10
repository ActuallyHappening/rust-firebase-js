use proc_macro2::TokenStream;

pub fn js_bind_impl(attrs: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
	Ok(input)
}