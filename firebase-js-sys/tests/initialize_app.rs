use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use wasm_bindgen_test::{wasm_bindgen_test as test};
use firebase_js_sys::app;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn manual_initialize_app_empty() {
	let result = app::initialize_app(&JsValue::UNDEFINED);
	
	assert!(result.is_err());
	panic!("Error description: {:?}", result.err().unwrap());
}
