use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::*;
use smart_default::SmartDefault;
use syn::*;
use syn::{parse::*, spanned::Spanned};

use crate::config::{Config, FromTOMLCwd};

#[derive(Debug, SmartDefault)]
struct JsBindAttrs {
	/// The module ref to import from
	module: BindVarient,

	options: BindOptions,
}

#[derive(Debug, SmartDefault)]
struct BindOptions {
	/// Whether to inject documentation regarding the function as imported from JS
	#[default(true)]
	doc: bool,
	/// Whether to take the documentation tests marked with `# JS_BIND_TEST` and put then in tests dir
	#[default(true)]
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

		match input.parse::<Token![,]>() {
			Ok(_) => {
				// There is a comma, keep parsing!
				attrs.options = BindOptions::parse(input)?;
			},
			Err(_) => {},
		}

		Ok(attrs)
	}
}

impl Parse for BindOptions {
	fn parse(input: ParseStream) -> Result<BindOptions> {
		let mut options = BindOptions::default();
		while !input.is_empty() {
			let option = input.parse::<Ident>()?;
			match option.to_string().as_str() {
				"doc" => options.doc = true,
				"test" => options.test = true,
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
		Ok(options)
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

fn get_functions(input: &ItemForeignMod) -> Vec<&ForeignItemFn> {
	let mut functions = Vec::new();
	for item in &input.items {
		if let ForeignItem::Fn(foreign_item_fn) = item {
			functions.push(foreign_item_fn);
		}
	}
	functions
}

fn find_js_bind_attr(input: &ForeignItemFn) -> syn::Result<Option<&Attribute>> {
	let mut js_bind_attr = None;
	for attr in &input.attrs {
		if attr.path().is_ident("js_bind") {
			if let Some(_) = js_bind_attr {
				// throw error
				Err(Error::new(
					attr.span(),
					"Multiple #[js_bind] attributes found on a single function",
				))?;
			}
			js_bind_attr = Some(attr);
		}
	}
	Ok(js_bind_attr)
}

/// Takes an attribute like `#[js_bind()]` and adds a `_mod` field to it
/// if not already present, like `#[js_bind(_mod = "foobar")]`
fn add_default_mod(input: &Attribute, module: String) -> syn::Result<Attribute> {
	assert_eq!(input.meta.path().segments[0].ident.to_string(), "js_bind");

	let list = input.meta.require_list()?;
	match list.parse_args::<JsBindAttrs>() {
		Ok(_) => {
			return Ok(input.clone());
		}
		Err(err_manual_mod) => {
			// Doesn't have `module` or `_mod`, we might be in luck!
			match list.parse_args::<BindOptions>() {
				Ok(_) => {
					// Yes, ready to fill in with `module` or `_mod`
					let mut attr = input.clone();

					let new_tokens = list.tokens.clone().into_iter().chain(
						quote! {
							_mod = #module,
						}
						.into_iter(),
					);
					let tokens = TokenStream::from_iter(new_tokens);
					let merged_tokens = TokenStream::from_iter(
						tokens
							.into_iter()
							.chain(list.tokens.clone().into_iter().skip(1)),
					);

					attr.meta = Meta::List(MetaList {
						path: list.path.clone(),
						delimiter: list.delimiter.clone(),
						tokens: merged_tokens,
					});

					return Ok(attr);
				}
				Err(err_no_mod) => {
					// Doesn't parse as anything valid, we must be in the wrong place
					let mut top_err = Error::new(
						input.span(),
						format!(
							r##"(From top level #[js_bind] macro:) Cannot parse my child macro's inputs to (potentially) add a `_mod` attribute.
Please make sure your use of #[js_bind] is valid here. The root error should be combined with this error, so see below."##
						),
					);
					top_err.combine(err_manual_mod);
					top_err.combine(err_no_mod);
					// top_err.to_compile_error();
					return Err(top_err);
				}
			}
		}
	}
}



// /// If passed `#[js_bind(module = "foobar")]` then assumes its input is a foreign block
// /// Else, assumes it is passed _mod = "foobar" and parses it as a function
// pub fn _js_bind_impl(
// 	attrs: proc_macro2::TokenStream,
// 	input: proc_macro2::TokenStream,
// ) -> proc_macro2::TokenStream {
// 	let attrs: JsBindAttrs = match syn::parse2(attrs) {
// 		Ok(syntax_tree) => syntax_tree,
// 		Err(err) => return proc_macro2::TokenStream::from(err.to_compile_error()),
// 	};

// 	if let BindVarient::TopLevel(module) = attrs.module {
// 		// TOP LEVEL
// 		let input: ItemForeignMod = match syn::parse2(input) {
// 			Ok(syntax_tree) => {
// 				// println!("Syntax tree: {:#?}", syntax_tree);
// 				syntax_tree
// 			}
// 			Err(err) => return TokenStream::from(err.to_compile_error()),
// 		};

// 		// loop through ever function in the foreign block
// 		// and add default `_mod` field if not already provided
// 		let functions = get_functions(&input);
// 		// println!("Found {} functions: {:?}", functions.len(), functions);
// 		for f in functions {
// 			println!("Found function! Name: {}", f.sig.ident.to_string());
// 			let attr = match find_js_bind_attr(f) {
// 				Ok(attr) => attr,
// 				Err(err) => return TokenStream::from(err.to_compile_error()),
// 			};
// 			match attr {
// 				Some(attr) => {
// 					// println!("Found attr: {:#?}", attr);
// 					match add_default_mod(attr, module.clone()) {
// 						Ok(new_attr) => {
// 							// TODO: Take new_attr and replace the old attr
// 						}
// 						Err(err) => return TokenStream::from(err.to_compile_error()),
// 					}
// 				}
// 				None => {
// 					println!("No attr found");
// 				}
// 			}
// 		}

// 		let config = Config::from_cwd().expect("Cannot parse config");

// 		let expanded = quote! {
// 			#input
// 		};

// 		expanded.into()
// 	} else {
// 		// Acting on a specific function / type inside wasm-bindgen
// 		let input: ForeignItemFn = match syn::parse2(input) {
// 			Ok(syntax_tree) => syntax_tree,
// 			Err(err) => return proc_macro2::TokenStream::from(err.to_compile_error()),
// 		};

// 		let expanded = quote! {
// 			// #input
// 		};

// 		expanded.into()
// 	}

// 	// let docs = extract_docs(&input.attrs);
// 	// eprintln!("Docs: {:#?}", docs);
// }
