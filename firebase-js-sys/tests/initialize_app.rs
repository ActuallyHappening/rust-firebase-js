use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use firebase_js_sys::app;

#[test]
fn manual_initialize_app_empty() {
	let result = app::initialize_app(&JsValue::UNDEFINED, None);
	
	assert!(result.is_err());
	// panic!("Error description: {:?}", result.err().unwrap());
}
