use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use firebase_js_sys::app;
use js_bind::duplicate_wasmbindgen_test as duplicate_test;

#[cfg(feature = "web-not-node")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn manual_initialize_app_empty() {
	let result = app::initialize_app(JsValue::UNDEFINED);
	
	assert!(result.is_err());
	// assert!(false);
	// panic!("Error description: {:?}", result.err().unwrap());
}