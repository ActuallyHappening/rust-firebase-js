use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test as test;
#[allow(unused_imports)]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use firebase_js_sys::app;

#[cfg(feature = "web-not-node")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Tests using values:
/// - Undefined
/// - Null
/// - ""
/// #- {}
fn test_stupid_values(test_target: impl Fn(JsValue) -> Result<JsValue, JsValue>, response_handler: impl Fn(JsValue, Result<JsValue, JsValue>) -> ()) {
	// Undefined values
	let result = test_target(JsValue::UNDEFINED);
	response_handler(JsValue::UNDEFINED, result);
	let result = test_target(JsValue::NULL);
	response_handler(JsValue::NULL, result);
	let result = test_target(JsValue::from(""));
	response_handler(JsValue::from(""), result);

	// test {}
	// let test_val = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
	// let result = test_target(test_val.clone());
	// response_handler(test_val.clone(), result);
}

#[test]
fn initialize_app() {
	// test_stupid_values(app::initialize_app, |test_val, results| {
	// 	assert!(results.is_err(), "Expected error when passed value {:?}, got {:?}", test_val, results);
	// 	// panic!("Error description: {:?}", results.err().unwrap());
	// })

	let config_obj = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
	let result = app::initialize_app(config_obj.clone());
	assert!(result.is_ok());
}