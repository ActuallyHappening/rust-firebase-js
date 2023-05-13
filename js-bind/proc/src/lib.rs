use proc_macro::TokenStream;
use js_bind_core::js_bind_impl;

#[proc_macro_attribute]
pub fn js_bind(attr: TokenStream, input: TokenStream) -> TokenStream {
	js_bind_impl(attr.into(), input.into()).map_or_else(|err| err.to_compile_error().into(), |output| output.into())
}

#[proc_macro_attribute]
pub fn duplicate_wasmbindgen_test(attr: TokenStream, input: TokenStream) -> TokenStream {
	js_bind_core::duplicate_wasmbindgen_test_impl(attr.into(), input.into()).map_or_else(|err| err.to_compile_error().into(), |output| output.into())
}