#![allow(dead_code)]
#![allow(unused_imports)]

use extract_doctests::extract_doctests;

pub fn exported_from_crate_root() -> i32 {
	42
}

/// Documentation
/// ```rust,should_panic
/// // extract-doctests name_of_func
///
///	use foobar::exported_from_crate_root;
/// 
/// assert_eq!(1, 1);
/// return exported_from_crate_root()
/// ```
#[extract_doctests(inline_config(template = r##"
fn {test_name}() -> i32 {
	{code}
}
"##, replace_package = "foobar"))]
pub fn placeholder_item() {}

/// Running `cargo run --example package_replace_usage` will print `5`
fn main() {
	let returned = name_of_func();
	println!("Returned: {}", returned);
	assert_eq!(returned, 42);
}
