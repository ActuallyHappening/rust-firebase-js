#![allow(dead_code)]
#![allow(unused_imports)]

use extract_doctests::extract_doctests;

/// Documentation
/// ```rust,should_panic
/// // extract-doctests name_of_func
///
/// // This statement is only executed by rustdoc because
/// // of the template that is used, see Cargo.toml
/// assert_eq!(1, 1);
/// return 5
/// ```
#[extract_doctests(inline_config(template = r##"
fn {test_name}() -> i32 {
	{code}
}
"##))]
pub fn placeholder_item() {}

/// Running `cargo run --example simple_usage` will print `5`
fn main() {
	let returned = name_of_func();
	println!("Returned: {}", returned);
	assert_eq!(returned, 5);
}
