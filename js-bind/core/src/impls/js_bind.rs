use convert_case::{Case, Casing};
use quote::*;
use smart_default::SmartDefault;
use syn::parse::*;
use syn::*;

use crate::config::{Config, FromTOMLCwd};

#[derive(Debug, Default)]
struct JsBindAttrs {
	/// If Some(_), implied that this macro is being used at the top level of an `extern` block
	/// If None, implied that this macro is being used at the top level of a function
	/// The module ref to import from
	module: BindVarient,

	/// Whether to inject documentation regarding the function as imported from JS
	doc: bool,
	/// Whether to take the documentation tests marked with `# JS_BIND_TEST` and put then in tests dir
	test: bool,
}

#[derive(Debug, SmartDefault)]
enum BindVarient {
	#[default]
	TopLevel(String),
	Function(String),
}

impl Parse for JsBindAttrs {
	/// Parse js_bind(module = "test/app", doc, test)
	fn parse(input: ParseStream) -> Result<Self> {
		let mut attrs = JsBindAttrs::default();
		let maybe_mod: Ident = input.parse()?;
		if maybe_mod.to_string().as_str() == "module" {
			input.parse::<Token![=]>()?;
			attrs.module = BindVarient::TopLevel(input.parse::<LitStr>()?.value());
		} else if maybe_mod.to_string().as_str() == "_mod" {
			input.parse::<Token![=]>()?;
			attrs.module = BindVarient::Function(input.parse::<LitStr>()?.value());
		} else {
			// Throw error
			return Err(Error::new(
				maybe_mod.span(),
				format!(
					r##"Unknown option: "{}"; Expected either "module" or "_mod"
					Have you forgotten to add `#[js_bind(module = "...")]` at the top level of your `extern "C" {{ ... }}` block?
					If not, have you forgotten to add `#[js_bind(_mod = "foobar", ...)]` at the top level of your function instead of `#[js_bind(...)]`?
					"##,
					maybe_mod.to_string()
				),
			));
		}

		while !input.is_empty() {
			let option = input.parse::<Ident>()?;
			match option.to_string().as_str() {
				"doc" => attrs.doc = true,
				"test" => attrs.test = true,
				_ => {
					return Err(Error::new(
						option.span(),
						format!(
							r##"Unknown option: "{}"; Expected either "doc" or "test""##,
							option.to_string()
						),
					))
				}
			}
			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(attrs)
	}
}

fn convert_from_snake_case_to_camel_case(name: String) -> String {
	name.to_case(Case::Camel)
}

fn extract_docs(attrs: &Vec<Attribute>) -> Vec<String> {
	let mut doc_comments = Vec::new();
	(attrs.clone()).into_iter().for_each(|attr| {
		// eprintln!("Attr: {:#?}", attr);
		if let Meta::NameValue(meta_name_value) = attr.meta {
			if meta_name_value.path.is_ident("doc") {
				match meta_name_value.value {
					Expr::Lit(ExprLit {
						lit: Lit::Str(doc), ..
					}) => {
						doc_comments.push(doc.value());
					}
					_ => {}
				}
			}
		}
	});
	doc_comments
}

/// If passed `#[js_bind(module = "foobar")]` then assumes its input is a foreign block
/// Else, assumes it is passed _mod = "foobar" and parses it as a function
pub fn _js_bind_impl(
	attrs: proc_macro2::TokenStream,
	input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
	let attrs: JsBindAttrs = match syn::parse2(attrs) {
		Ok(syntax_tree) => syntax_tree,
		Err(err) => return proc_macro2::TokenStream::from(err.to_compile_error()),
	};

	if let BindVarient::TopLevel(module) = attrs.module {
		let input: ItemForeignMod = match syn::parse2(input) {
			Ok(syntax_tree) => {
				println!("Syntax tree: {:#?}", syntax_tree);
				syntax_tree
			}
			Err(err) => return proc_macro2::TokenStream::from(err.to_compile_error()),
		};

		let config = Config::from_cwd().expect("Cannot parse config");

		let expanded = quote! {
			#input
		};

		expanded.into()
	} else {
		// Acting on a specific function / type inside wasm-bindgen
		let input: ItemFn = match syn::parse2(input) {
			Ok(syntax_tree) => syntax_tree,
			Err(err) => return proc_macro2::TokenStream::from(err.to_compile_error()),
		};

		let expanded = quote! {
			// #input
		};

		expanded.into()
	}

	// let docs = extract_docs(&input.attrs);
	// eprintln!("Docs: {:#?}", docs);
}

// pub fn _js_bind_impl(_attr: proc_macro2::TokenStream, _input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
// 	let input: ItemFn = syn::parse2(_input).expect("Cannot parse input as a function");
// 	let attr: JsBindAttrs = syn::parse2(_attr).expect(r##"Cannot parse attributes as `method = "something"`"##);

// 	let cwd = std::env::current_dir().expect("Cannot get current working directory");
// 	let config = Config::from_config_dir(&cwd).expect("Cannot parse config");
// 	let mode = config.modes.get(&attr.mode).expect(&format!(r##"Cannot find mode "{}" in config"##, &attr.mode));
// 	let mut lock = ConfigLock::from_config_dir(&cwd).expect("Cannot parse config lock");

// 	let func_name = input.sig.ident.to_string();
// 	let mode_name = attr.mode;
// 	let func: Function = Function::new(func_name, mode_name);
// 	lock.append_func(&cwd,	func).expect("Cannot add function to config lock");

// 	// quote!{pub fn works() -> i32 {42}}.into()
// 	quote!{}.into()
// }

// pub fn _js_bind_impl2(attr: proc_macro2::TokenStream, input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
// 	let item = syn::parse::<ItemFn>(input.into()).map_err(|e| e.to_compile_error()).expect("ItemFn to parse properly");
// 	// eprintln!("Item: {:#?}", item);

// 	let sig = &item.sig;
// 	let sig_str = quote!(#sig);
// 	// eprintln!("Sig str: {:?}", sig_str.to_string());
// 	let sig_name = &sig.ident;

// 	// handle_doc_comments(&item);

// 	// let attr = parse_macro_input!(attr as LitStr);
// 	// let attr_indent = format_ident!("{}", attr.value());
// 	// // eprintln!("Attr: {:#?}", attr);
// 	// let module_name = attr_indent;
// 	// let module_name_underscore = format_ident!("_{}", &module_name);

// 	// let attrs = parse_macro_input!(attr as JsBindAttrs);
// 	let attrs = syn::parse::<JsBindAttrs>(attr.into()).map_err(|e| e.to_compile_error()).expect("ItemFn to parse properly");

// 	// eprintln!("Attr: {:#?}", attrs);
// 	let js_mod_name = Ident::new(&attrs.js_mod_name, sig.span());
// 	let js_mod_name_str = format!(r"{}", js_mod_name.to_string());
// 	let _js_mod_name = format_ident!("_{}", &js_mod_name);
// 	let js_method_name_str = format!(r#"{}"#, attrs.js_method_name.unwrap_or(convert_from_snake_case_to_camel_case(sig_name.to_string())));

// 	let sig_inputs = &sig.inputs;
// 	// eprintln!("Sig inputs: {:#?}", sig_inputs);

// 	let passed_parameters = sig_inputs.iter().map(|arg| {
// 		match arg {
// 			FnArg::Receiver(_) => panic!("Cannot use receiver in js_bind"),
// 			FnArg::Typed(pat_type) => {
// 				let pat = &pat_type.pat;
// 				// let ty = &pat_type.ty;
// 				// quote!(#pat: #ty)
// 				quote!(#pat)
// 			}
// 		}
// 	});

// 	let function_wrapper = quote! {
// 		pub #sig_str {
// 			// #module_name_underscore::#attr_indent(#(#sig_inputs),*)
// 			// #module_name_underscore::#sig_name()
// 			// #[cfg(feature = "verbose-logging")]
// 			::log::info!("Calling function: {}::{}(<parameters pass not implemented yet>)", stringify!(#_js_mod_name), stringify!(#sig_name));
// 			#_js_mod_name::#sig_name(#(#passed_parameters),*)
// 		}
// 	};

// 	let _internal_docs = format!(r##"This is an internal function, generated by the #[js_bind] macro. By design, the module is private"##);
// 	let internal_docs = quote! {
// 		#[doc = #_internal_docs]
// 	};

// 	#[cfg(feature = "strict-feature-checks")]
// 	let feature_checks = quote! {
// 		// If no feature enabled
// 		#[cfg(all(not(feature = "node-not-web"), not(feature = "web-not-node")))]
// 		compile_error!("Must enable either feature `web-not-node` or `node-not-web`");
// 		// If both features enabled
// 		#[cfg(all(feature = "node-not-web", feature = "web-not-node"))]
// 		compile_error!("Most not enable both features `web-not-node` and `node-not-web`");
// 	};
// 	#[cfg(not(feature = "strict-feature-checks"))]
// 	let feature_checks = quote! {
// 		// If no feature enabled
// 		#[cfg(all(not(feature = "node-not-web"), not(feature = "web-not-node")))]
// 		eprintln!("[strict-feature-checks=false => no error] Must enable either feature `web-not-node` or `node-not-web`");
// 		// If both features enabled
// 		#[cfg(all(feature = "node-not-web", feature = "web-not-node"))]
// 		eprintln!("[strict-feature-checks=false => no error] Most not enable both features `web-not-node` and `node-not-web`");
// 	};

// 	let expanded = quote! {
// 		#feature_checks

// 		// use wasm_bindgen::prelude::wasm_bindgen;
// 		// If either feature is enabled
// 		#[cfg_attr(
// 			all(feature = "web-not-node", not(feature = "node-not-web")),
// 			::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-es.js")
// 		)]
// 		#[cfg_attr(
// 			all(feature = "node-not-web", not(feature = "web-not-node")),
// 			::wasm_bindgen::prelude::wasm_bindgen(module = "/js/bundle-cjs.js")
// 		)]
// 		extern "C" {
// 			#[allow(non_camel_case_types)]
// 			#[::wasm_bindgen::prelude::wasm_bindgen(js_name = #js_mod_name_str)]
// 			type #_js_mod_name;

// 			#[::wasm_bindgen::prelude::wasm_bindgen(catch, static_method_of = #_js_mod_name, js_name = #js_method_name_str)]
// 			#internal_docs
// 			#sig_str;
// 		}

// 		#function_wrapper
// 	};

// 	// eprintln!("Expanded: {}", expanded.to_string());

// 	expanded.into()
// }
