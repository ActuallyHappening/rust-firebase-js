use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, format_ident};
use syn::{parse_macro_input, LitStr, FnArg, ItemFn, Meta, MetaList, Result, parse::{ParseStream, Parse}, spanned::Spanned};

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

#[derive(Debug)]
struct JsBindAttrs {
	js_mod_name: String,
	js_method_name: Option<String>,
}

impl Parse for JsBindAttrs {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			js_mod_name: input.parse::<LitStr>()?.value(),
			js_method_name: input.parse::<LitStr>().ok().map(|s| s.value()),
		})
	}
}

fn convert_from_snake_case_to_camel_case(name: String) -> String {
	name.to_case(Case::Camel)
}

/// Binds a regular function signature using wasm-bindgen
#[proc_macro_attribute]
pub fn js_bind(attr: TokenStream, input: TokenStream) -> TokenStream {

	let item = parse_macro_input!(input as ItemFn);
	// eprintln!("Item: {:#?}", item);
	let sig = &item.sig;
	let sig_str = quote!(#sig);
	// eprintln!("Sig str: {:?}", sig_str.to_string());
	let sig_name = &sig.ident;

	// let attr = parse_macro_input!(attr as LitStr);
	// let attr_indent = format_ident!("{}", attr.value());
	// // eprintln!("Attr: {:#?}", attr);
	// let module_name = attr_indent;
	// let module_name_underscore = format_ident!("_{}", &module_name);

	let attrs = parse_macro_input!(attr as JsBindAttrs);
	eprintln!("Attr: {:#?}", attrs);
	let js_mod_name = Ident::new(&attrs.js_mod_name, sig.span());
	let js_mod_name_str = format!(r"{}", js_mod_name.to_string());
	let _js_mod_name = format_ident!("_{}", &js_mod_name);
	let js_method_name_str = format!(r#"{}"#, attrs.js_method_name.unwrap_or(convert_from_snake_case_to_camel_case(sig_name.to_string())));

	let sig_inputs = &sig.inputs;
	// eprintln!("Sig inputs: {:#?}", sig_inputs);

	let passed_parameters = sig_inputs.iter().map(|arg| {
		match arg {
			FnArg::Receiver(_) => panic!("Cannot use receiver in js_bind"),
			FnArg::Typed(pat_type) => {
				let pat = &pat_type.pat;
				// let ty = &pat_type.ty;
				// quote!(#pat: #ty)
				quote!(#pat)
			}
		}
	});

	let function_wrapper = quote! {
		#sig_str {
			// #module_name_underscore::#attr_indent(#(#sig_inputs),*)
			// #module_name_underscore::#sig_name()
			// #[cfg(feature = "verbose-logging")]
			::log::info!("Calling function: {}::{}(<parameters pass not implemented yet>)", stringify!(#_js_mod_name), stringify!(#sig_name));
			#_js_mod_name::#sig_name(#(#passed_parameters),*)
		}
	};

	eprintln!("Function wrapper: {}", function_wrapper.to_string());

	let expanded = quote! {
		// If no feature enabled
		#[cfg(all(not(feature = "node-not-web"), not(feature = "web-not-node")))]
		compile_error!("Must enable either feature `web-not-node` or `node-not-web`");

		// use wasm_bindgen::prelude::wasm_bindgen;
		// If either feature is enabled
		#[cfg_attr(
			all(feature = "web-not-node", not(feature = "node-not-web")),
			::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-es.js")
		)]
		#[cfg_attr(
			all(feature = "node-not-web", not(feature = "web-not-node")),
			::wasm_bindgen::prelude::wasm_bindgen(module = "/target/js/bundle-cjs.js")
		)]
		extern "C" {
			#[allow(non_camel_case_types)]
			#[::wasm_bindgen::prelude::wasm_bindgen(js_name = #js_mod_name_str)]
			type #_js_mod_name;

			#[::wasm_bindgen::prelude::wasm_bindgen(catch, static_method_of = #_js_mod_name, js_name = #js_method_name_str)]
			#sig_str;
		}

		#function_wrapper
	};

	eprintln!("Expanded: {}", expanded.to_string());

	expanded.into()
}
