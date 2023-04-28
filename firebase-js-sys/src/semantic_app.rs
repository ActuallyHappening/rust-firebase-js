use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/target/js/bundle-es.js"))]
#[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/target/js/bundle-cjs.js"))]
extern "C" {
	#[allow(non_camel_case_types)]
	pub type app;

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
	#[wasm_bindgen(catch, static_method_of = app, js_name = "initializeApp")]
	pub fn initialize_app(config: &JsValue, name: Option<String>) -> Result<JsValue, JsValue>;
}