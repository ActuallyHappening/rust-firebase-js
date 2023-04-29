use proc_macro::TokenStream;
use quote::{quote, format_ident};
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

/// Binds a regular function signature using wasm-bindgen
#[proc_macro_attribute]
pub fn js_bind(attr: TokenStream, input: TokenStream) -> TokenStream {
	let attr = parse_macro_input!(attr as LitStr);
	let attr_indent = format_ident!("{}", attr.value());

	eprintln!("Attr: {:#?}", attr);

	let item = parse_macro_input!(input as ItemFn);
	let sig = &item.sig;
	let sig_str = quote!(#sig);
	eprintln!("Sig str: {:?}", sig_str.to_string());
	let sig_name = &sig.ident;

	let module_name = attr_indent;
	let module_name_underscore = format_ident!("_{}", &module_name);

	let sig_inputs = &sig.inputs;
	eprintln!("Sig inputs: {:#?}", sig_inputs);

	let parameters = sig_inputs.iter().map(|arg| {
		match arg {
			FnArg::Receiver(_) => panic!("Cannot use receiver in js_bind"),
			FnArg::Typed(pat_type) => {
				let pat = &pat_type.pat;
				let ty = &pat_type.ty;
				quote!(#pat: #ty)
			}
		}
	});

	let function_wrapper = quote! {
		#sig_str {
			// #module_name_underscore::#attr_indent(#(#sig_inputs),*)
			#module_name_underscore::#sig_name();
		}
	};

	let expanded = quote! {
		use wasm_bindgen::prelude::wasm_bindgen;
		#[cfg_attr(
			feature = "web-not-node",
			::wasm_bindgen(module = "/target/js/bundle-es.js")
		)]
		#[cfg_attr(
			feature = "node-not-web",
			::wasm_bindgen(module = "/target/js/bundle-cjs.js")
		)]
		#[wasm_bindgen]
		extern "C" {
			#[allow(non_camel_case_types)]
			#[::wasm_bindgen(js_name = #module_name)]
			type #module_name_underscore;

			#[::wasm_bindgen(catch, static_method_of = #module_name_underscore, js_name = "initializeApp")]
			#sig_str;
		}

		#function_wrapper
	};

	eprintln!("Expanded: {}", expanded.to_string());

	expanded.into()
}
