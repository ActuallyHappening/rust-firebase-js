#![allow(dead_code, improper_ctypes)]

use js_bind_proc::js_bind;

// This uses wasm bindgen!
#[js_bind(config_path = "examples/testing-configs/testing-flag-enabled.toml", conditional_attrs)]
// #[wasm_bindgen]
extern "C" {
	fn alert(s: &str);
}

fn main() {
	// These lines will give you an error, since the extern block is not using wasm_bindgen
	// alert_bad1("Hello, world!");
	// alert_bad2("Hello, world!");

	alert("Hello, world!");
}