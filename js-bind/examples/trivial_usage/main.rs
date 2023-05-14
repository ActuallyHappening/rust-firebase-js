#![allow(dead_code, improper_ctypes)]

use js_bind_proc::js_bind;

// // This uses wasm bindgen!
// #[js_bind(config_path = "examples/testing-configs/testing-flag-enabled.toml", fallback, conditional_attrs)]
// extern "C" {
// 	/// Documentation of func
// 	fn alert1(s: &str);
// }

// #[js_bind(config_path = "examples/testing-configs/testing-flag-enabled.toml", fallback, conditional_attrs)]
// extern "C" {
// 	/// Documentation of func
// 	/// 
// 	/// ```rust
// 	/// assert_eq!("Yes this test executed well!", "")
// 	/// ```
// 	fn alert2(s: &str);
// }

#[js_bind(config_path = "examples/testing-configs/firebase.js-bind.toml", fallback, extract_tests)]
extern "C" {
	/// Documentation of func
	/// 
	/// ```rust
	/// // JSBIND-TEST example_test_name
	/// assert_eq!("Yes this test executed well!", "")
	/// ```
	#[wasm_bindgen(js_name = "alert")]
	fn alert3(s: &str);
}

fn main() {
	// alert1("Hello, world!");
	// alert2("Hello, world!");
	alert3("Hello, world!");
}