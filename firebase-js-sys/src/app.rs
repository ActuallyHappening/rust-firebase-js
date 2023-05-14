use js_bind::js_bind;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[js_bind(config_path = "firebase-js-sys/js-bind.toml", conditional_attrs, extract_tests)]
// #[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/js/bundle-esm.js"))]
// #[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/js/bundle-cjs.js"))]
// #[wasm_bindgen]
extern "C" {
	/// Takes a config object and returns a firebase app instance
	///
	/// Equivalent to:
	/// ```js
	/// import { initializeApp } from 'firebase/app';
	///
	/// // Get your own config from somewhere, typically copy-paste from firebase console
	/// const config = {
	/// 	apiKey: "...",
	/// 	projectId: "...",
	/// 	...
	/// }
	///
	/// initializeApp(config);
	/// ```
	/// 
	/// ## Examples
	/// ```rust
	/// use firebase_js_sys::app;
	/// use wasm_bindgen::JsValue;
	/// 
	/// let config = JsValue::UNDEFINED;
	/// let returned = app::initialize_app(config);
	/// 
	/// assert!(returned.is_err());
	/// ```
	#[wasm_bindgen(js_name = "initializeApp", catch)]
	pub fn initialize_app(config: JsValue) -> Result<JsValue, JsValue>;
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[cfg(feature = "web-not-node")]
// 	wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// 	#[wasm_bindgen_test::wasm_bindgen_test]
// 	fn test() {
// 		let config_obj = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
// 		let result = initialize_app(config_obj.clone());
// 		assert!(result.is_ok(), "Expected Ok, got {:?}", result);
// 	}
// }

