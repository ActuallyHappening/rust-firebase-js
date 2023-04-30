use quote::*;

pub fn _target_name_impl(_input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	quote! {
		match (cfg!(feature = "web-not-node"), cfg!(feature = "node-not-web")) {
			(true, false) => "web",
			(false, true) => "node",
			_ => panic!("Invalid target features set: Must be mutually exclusive and one enabled; web-not-node: {:?}, node-not-web {:?}", cfg!(feature = "web-not-node"), cfg!(feature = "node-not-web")),
		}
	}.into()
}
