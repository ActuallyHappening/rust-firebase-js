use proc_macro::TokenStream;
use js_bind_core::_js_bind_impl;

// #[proc_macro]
// pub fn target_name(input: TokenStream) -> TokenStream {
// 	_target_name_impl(input.into()).into()
// }

/// Binds a regular function signature using wasm-bindgen
#[proc_macro_attribute]
pub fn js_bind(attr: TokenStream, input: TokenStream) -> TokenStream {
	_js_bind_impl(attr.into(), input.into()).map_or_else(|e| e.into(), Error::into_compile_error)
}
