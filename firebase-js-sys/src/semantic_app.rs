use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
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
	#[wasm_bindgen(catch, static_method_of = app, js_name = "initializeApp")]
	pub fn initialize_app(config: &JsValue) -> Result<JsValue, JsValue>;
}
