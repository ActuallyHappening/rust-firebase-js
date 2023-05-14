use wasm_bindgen_test::wasm_bindgen_test as test;
#[allow(unused_imports)]
use wasm_bindgen_test::wasm_bindgen_test_configure;
use firebase_js_sys::app;

#[cfg(feature = "web-not-node")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn test_integration_initialize_app() {
	// test_stupid_values(app::initialize_app, |test_val, results| {
	// 	assert!(results.is_err(), "Expected error when passed value {:?}, got {:?}", test_val, results);
	// 	// panic!("Error description: {:?}", results.err().unwrap());
	// })

	let config_obj = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
	let result = app::initialize_app(config_obj.clone());
	assert!(result.is_ok());
}