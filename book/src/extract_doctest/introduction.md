# `extract-doctest`: Macro to extract the doctest using a procedural attribute macro
This package exports a single item: a macro `#[extract_doctest]`.
This macro then extracts the documentation tests found within its input, interpolates
it into a template, and then outputs the result.
Currently, only outputing functions is supported (because this is the safest use case).


## Example:
First, add a template to your package's Cargo.toml:
```toml
[package.metadata.extract-doctest]
template = """
// This rust code is added after the component macro invocation

// This specific template outputs a wasm32 unit-test function, that if
// the feature "web-not-node" is enabled, will tell `wasm-bindgen-test`
// to run the test in the browser.
// Of course you can do whatever you like, but my purpose of this crate
// is to write documentation test for wasm32 functions.
#[cfg(test)]
#[::wasm_bindgen_test::wasm_bindgen_test]
fn {test_name}() {
	#[cfg(feature = "web-not-node")]
	::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

	{code}
}
"""
# Optional
# Replaces a `use example_package_name::something` statement with `use crate::something`
# when interpolting the {code} variable (see template below)
# This is because the generated code is in the same crate as the `example_package_name` crate
package-to-crate = "example_package_name"
```
This config is used every #[extract_doctest] call.
Now you can use the macro:
```rust
use extract_doctest::extract_doctest;

/// Some documentation
/// # Example
/// ```rust
/// // this is a normal rust documentation test, which you can run
/// // with `cargo test`
/// assert_eq!(42, 42);
/// ```
/// The 'no_run' is ignored by extract-doctest, but is still
/// valid rust code and is interpreted by normal `rustdoc`
/// and `cargo test` invocations.
/// 
/// If this next example was to actually run through normal `cargo test`,
/// it would panic with `cannot call wasm-bindgen imported functions on
/// non wasm32 targets`.
/// 
/// ```rust,no_run
/// // extract-doctest: name_of_test
/// // ^^^^^^^^^^^^^^^ MUST be there
/// // MUST be there    ^^^^^^^^^^^^ < is the name of the test generated
/// 
/// // The above comment MUST be on the first line after the ```
/// // that begins the code block.
/// // Else, how would #[extract_doctest] know which tests to extract?
/// 
/// #[wasm_bindgen]
/// extern "C" {
/// 	#[wasm_bindgen(js_namespace = console)]
/// 	fn log(s: &str);
/// }
/// 
/// log("This is running on a wasm target!");
/// ```
#[extract_doctest]
pub fn show_wasm_alert() {
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_namespace = console)]
		fn alert(s: &str);
	}

	alert("[show_wasm_alert fn]: The function is executed!");
}
```