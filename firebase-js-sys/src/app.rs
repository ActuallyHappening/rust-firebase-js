use extract_doctests::extract_doctests;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[extract_doctests]
#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/js/bundle-esm.js"))]
#[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/js/bundle-cjs.js"))]
// #[wasm_bindgen]
extern "C" {
	/// Takes a config object and returns a firebase app instance.
	///
	/// Equivalent to:
	/// ```js
	/// import { initializeApp } from 'firebase/app';
	///
	/// // Get your own config from somewhere, typically copy-paste from firebase console
	/// const config = {
	/// 	projectId: "...",
	/// 	apiKey: "...",
	/// }
	///
	/// initializeApp(config);
	/// ```
	///
	/// ## Examples
	/// Raw example, asserting various known properties
	/// ```rust,no_run
	/// // extract-doctests initialize_app
	/// use firebase_js_sys::app::initialize_app;
	/// use wasm_bindgen::JsValue;
	///
	/// // Should Error
	/// assert!(initialize_app(JsValue::UNDEFINED, JsValue::UNDEFINED).is_err());
	/// assert!(initialize_app(JsValue::NULL, JsValue::UNDEFINED).is_err());
	/// assert!(initialize_app(serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap(), JsValue::from_str("")).is_err());
	///
	/// // Should not Error
	/// assert!(initialize_app(serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap(), JsValue::UNDEFINED).is_ok());
	/// assert!(initialize_app(serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap(), JsValue::from_str("project name here")).is_ok());
	/// ```
	#[wasm_bindgen(js_name = "initializeApp", catch)]
	pub fn initialize_app(config: JsValue, optional_name: JsValue) -> Result<JsValue, JsValue>;
}
