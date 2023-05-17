#![allow(dead_code)]
#![allow(unused_imports)]

use extract_doctests::extract_doctests;
use wasm_bindgen::prelude::*;

#[extract_doctests(inline_config(template = r##"
fn {test_name}() -> i32 {
	{code}
}
"##))]
#[wasm_bindgen]
extern "C" {
	/// Documentation
	/// ```rust,should_panic
	/// // extract-doctests name_of_func
	///
	/// // This statement is only executed by rustdoc because
	/// // of the template that is used, see Cargo.toml
	/// assert_eq!(1, 1);
	/// return 5
	/// ```
	fn placeholder_item();
}

#[extract_doctests(inline_config(template = r##"
fn {test_name}() -> i32 {
	{code}
}
"##))]
#[wasm_bindgen(module = "/examples/placeholder.js")]
extern "C" {
	/// Documentation
	/// ```rust,should_panic
	/// // extract-doctests name_of_func2
	///
	/// // This statement is only executed by rustdoc because
	/// // of the template that is used, see Cargo.toml
	/// assert_eq!(1, 1);
	/// return 42
	/// ```
	pub fn placeholder_item2();
}

/// Running `cargo run --example wasmbindgen_usage` will print `5`
fn main() {
	let returned = name_of_func();
	let returned2 = name_of_func2();
	println!("Returned: {}, {}", returned, returned2);
	assert_eq!(returned, 5);
	assert_eq!(returned2, 42);
}
