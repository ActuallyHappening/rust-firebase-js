use extract_doctest_core::extract_doctest_impl;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn extract_doctest(attr: TokenStream, input: TokenStream) -> TokenStream {
	extract_doctest_impl(attr.into(), input.into())
		.map_or_else(|err| err.to_compile_error().into(), |i| i.into())
}
