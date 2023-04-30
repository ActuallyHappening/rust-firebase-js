use convert_case::{Case, Casing};

use proc_macro2::Ident;
use quote::{quote, format_ident};
use syn::{parse_macro_input, LitStr, FnArg, ItemFn, Meta, MetaList, Result, parse::{ParseStream, Parse}, spanned::Spanned, Expr, ExprLit, Lit};

pub fn _target_name_impl(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeContext {
// 	function_name: String,
// 	js_function_name: String,
// 	js_module_name: String,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeBlock {
// 	lines: Vec<String>,
// 	parameters: CodeParams,
// 	context: CodeContext,
// }

// impl CodeBlock {
// 	pub fn new(lines: Vec<String>, context: CodeContext) -> CodeBlock {
// 		Self {
// 			// TODO: not clone for performance?
// 			lines: lines.clone(),
// 			parameters: CodeParams::from_strings(lines),
// 			context,
// 		}
// 	}
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeParams {
// 	lang_identifier: Option<String>,
// 	test_option: Option<String>,
// 	other: Option<Vec<String>>
// }

// impl CodeParams {
// 	/// Transforms a string vector into a CodeParams struct
// 	/// 
// 	pub fn from_strings(start_line: String) -> CodeParams {
// 		let items: Vec<&str> = start_line.trim().to_owned().split(",").collect();
// 		Self {
// 			// TODO: Not clone for performance?
// 			lang_identifier: items.first().map(|f| f.to_owned()),
// 			test_option: items.get(1).cloned(),
// 			other: items.get(2..).map(|v| v.to_vec()),
// 		}
// 	}
// }

fn _handle_doc(doc: Vec<String>) {
	// eprintln!("Doc: {:?}", doc);

	// Extract ```rs ``` code blocks
	let mut code_blocks: Vec<Vec<String>> = Vec::new();
	let mut in_code_block = false;
	let mut code_block: Vec<String> = Vec::new();
	for line in doc {
		let line = line.trim();
		if line.starts_with("```") {
			if in_code_block {
				// Exiting code block
				code_block.push(line.to_owned());
				code_blocks.push(code_block);
				code_block = Vec::new();
				in_code_block = false;
			} else {
				// Entering code block
				code_block = Vec::new(); // Repetition of line 4 above
				code_block.push(line.to_owned());
				in_code_block = true;
			}
		} else if in_code_block {
			// In code block, not boundary
			code_block.push(line.to_owned());
		}
	}

	eprintln!("Code blocks: {:#?}", code_blocks);
}

fn handle_doc_comments(func: &ItemFn) {
	// TODO: Not clone? maybe expensive?
	let mut doc_comments = Vec::new();
	(func.attrs.clone()).into_iter().for_each(|attr| {
		eprintln!("Attr: {:#?}", attr);
		if let Meta::NameValue(meta_name_value) = attr.meta {
			if meta_name_value.path.is_ident("doc") {
				match meta_name_value.value {
					Expr::Lit(ExprLit { lit: Lit::Str(doc), .. }) => {
						doc_comments.push(doc.value());
					},
					_ => {}
				}
			}
		}
	});
	_handle_doc(doc_comments);
}

pub fn _js_bind_impl(attr: proc_macro2::TokenStream, input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	// let attr: proc_macro::TokenStream = proc_macro::TokenStream::from(attr);
	// let input: proc_macro::TokenStream = proc_macro::TokenStream::from(input);
	// let item = parse_macro_input!(input as ItemFn);
	let item = syn::parse::<ItemFn>(input.into()).map_err(|e| e.to_compile_error()).expect("ItemFn to parse properly");
	// eprintln!("Item: {:#?}", item);
	let sig = &item.sig;
	let sig_str = quote!(#sig);
	// eprintln!("Sig str: {:?}", sig_str.to_string());
	let sig_name = &sig.ident;


	handle_doc_comments(&item);

	// let attr = parse_macro_input!(attr as LitStr);
	// let attr_indent = format_ident!("{}", attr.value());
	// // eprintln!("Attr: {:#?}", attr);
	// let module_name = attr_indent;
	// let module_name_underscore = format_ident!("_{}", &module_name);

	// let attrs = parse_macro_input!(attr as JsBindAttrs);
	let attrs = syn::parse::<JsBindAttrs>(attr.into()).map_err(|e| e.to_compile_error()).expect("ItemFn to parse properly");
	
	// eprintln!("Attr: {:#?}", attrs);
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
		pub #sig_str {
			// #module_name_underscore::#attr_indent(#(#sig_inputs),*)
			// #module_name_underscore::#sig_name()
			// #[cfg(feature = "verbose-logging")]
			::log::info!("Calling function: {}::{}(<parameters pass not implemented yet>)", stringify!(#_js_mod_name), stringify!(#sig_name));
			#_js_mod_name::#sig_name(#(#passed_parameters),*)
		}
	};

	let _internal_docs = format!(r##"This is an internal function, generated by the #[js_bind] macro. By design, the module is private"##);
	let internal_docs = quote! {
		#[doc = #_internal_docs]
	};

	#[cfg(feature = "strict-feature-checks")]
	let feature_checks = quote! {
		// If no feature enabled
		#[cfg(all(not(feature = "node-not-web"), not(feature = "web-not-node")))]
		compile_error!("Must enable either feature `web-not-node` or `node-not-web`");
		// If both features enabled
		#[cfg(all(feature = "node-not-web", feature = "web-not-node"))]
		compile_error!("Most not enable both features `web-not-node` and `node-not-web`");
	};
	#[cfg(not(feature = "strict-feature-checks"))]
	let feature_checks = quote! {
		// If no feature enabled
		#[cfg(all(not(feature = "node-not-web"), not(feature = "web-not-node")))]
		eprintln!("[strict-feature-checks=false => no error] Must enable either feature `web-not-node` or `node-not-web`");
		// If both features enabled
		#[cfg(all(feature = "node-not-web", feature = "web-not-node"))]
		eprintln!("[strict-feature-checks=false => no error] Most not enable both features `web-not-node` and `node-not-web`");
	};

	let expanded = quote! {
		#feature_checks

		// use wasm_bindgen::prelude::wasm_bindgen;
		// If either feature is enabled
		#[cfg_attr(
			all(feature = "web-not-node", not(feature = "node-not-web")),
			::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-es.js")
		)]
		#[cfg_attr(
			all(feature = "node-not-web", not(feature = "web-not-node")),
			::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-cjs.js")
		)]
		extern "C" {
			#[allow(non_camel_case_types)]
			#[::wasm_bindgen::prelude::wasm_bindgen(js_name = #js_mod_name_str)]
			type #_js_mod_name;

			#[::wasm_bindgen::prelude::wasm_bindgen(catch, static_method_of = #_js_mod_name, js_name = #js_method_name_str)]
			#internal_docs
			#sig_str;
		}

		#function_wrapper
	};

	// eprintln!("Expanded: {}", expanded.to_string());

	expanded.into()
}