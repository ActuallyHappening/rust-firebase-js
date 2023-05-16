use extract_doctests_core::extract_doctests_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn extract_doctests(attr: TokenStream, input: TokenStream) -> TokenStream {
	extract_doctests_impl(attr.into(), input.into())
		.map_or_else(|err| err.to_compile_error().into(), |i| i.into())
}
