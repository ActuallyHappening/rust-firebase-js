use proc_macro::TokenStream;
use quote::quote;
use syn::*;

// mod _debug;
// #[proc_macro]
// pub fn test(_input: TokenStream) -> TokenStream {
// 	_debug::debug_impl(_input)
// }

#[proc_macro]
pub fn target_name(_input: TokenStream) -> TokenStream {
	quote! {
		match (cfg!(feature = "web-not-node"), cfg!(feature = "node-not-web")) {
			(true, false) => "web",
			(false, true) => "node",
			_ => panic!("Invalid target features set: Must be mutually exclusive and one enabled; web-not-node: {:?}, node-not-web {:?}", cfg!(feature = "web-not-node"), cfg!(feature = "node-not-web")),
		}
	}.into()
}

#[proc_macro_attribute]
pub fn nothing(attr: TokenStream, input: TokenStream) -> TokenStream {
	// panic!("Attr: {:?}\nItem: {:?}", attr, input);
	// eprintln!("Attr: {:?}", attr);
	eprintln!("Item: {:?}", input);
	
	quote!{}.into()
}

#[proc_macro]
pub fn duplicate_test(input: TokenStream) -> TokenStream {
	// panic!("Attr: {:?}\nItem: {:?}", attr, input);
	// eprintln!("Attr: {:?}", attr);
	eprintln!("Item: {:?}", input);
	let _input = parse_macro_input!(input as DeriveInput);

	let module_name = "_app";
	let js_module_name = "app";

	let expanded = quote! {
		#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/target/js/bundle-es.js"))]
		#[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/target/js/bundle-cjs.js"))]
		extern "C" {
			#[allow(non_camel_case_types)]
			#[wasm_bindgen(js_name = #js_module_name)]
			pub type #module_name;

			/// Takes a config object and returns a firebase app instance
			///
			/// Equivalent to:
			/// ```js
			/// import { initializeApp } from 'firebase/app';
			///
			/// // Get your own config from somewhere, typically copy-paste from firebase console
			/// const config = {
			/// 	apiKey: "...",
			/// 	projectId: "...",
			/// 	...
			/// }
			///
			/// initializeApp(config);
			/// ```
			/// 
			#[wasm_bindgen(catch, static_method_of = #, js_name = "initializeApp")]
			pub fn initialize_app(config: &JsValue, name: Option<String>) -> Result<JsValue, JsValue>;
		}
	};

	expanded.into()
}