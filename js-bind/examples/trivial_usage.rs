#![allow(dead_code, improper_ctypes)]

use js_bind_proc::js_bind;
use wasm_bindgen::prelude::*;

// WARNING: THIS DOES NOT USE WASMBINDGEN
#[js_bind]
extern "C" {
	// #[wasm_bindgen] // Adding this should error
	fn alert_bad1(s: &str);
}

// WARNING: THIS DOES NOT USE WASMBINDGEN
#[js_bind()]
extern "C" {
	fn alert_bad2(s: &str);
}

// This uses wasm bindgen!
#[js_bind(conditional_attrs)]
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