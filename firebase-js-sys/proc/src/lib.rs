use proc_macro::TokenStream;
use firebase_js_sys_proc_impl::*;

#[proc_macro]
pub fn target_name(_input: TokenStream) -> TokenStream {
	target_name(input.into()).into()
	
}

/// Binds a regular function signature using wasm-bindgen
#[proc_macro_attribute]
pub fn js_bind(attr: TokenStream, input: TokenStream) -> TokenStream {

	
}
