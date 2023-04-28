use proc_macro::TokenStream;
use quote::quote;
use syn::*;


#[proc_macro_attribute]
pub fn duplicate_test(attr: TokenStream, input: TokenStream) -> TokenStream {
	// panic!("Attr: {:?}\nItem: {:?}", attr, input);
	eprint!("Attr: {:?}", attr);
	eprintln!("Item: {:?}", input);
	let input = parse_macro_input!(input as DeriveInput);

	let expanded = quote! {
		#input
	};

	expanded.into()
}