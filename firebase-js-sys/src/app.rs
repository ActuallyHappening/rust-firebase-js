use firebase_js_sys_proc::js_bind;
use wasm_bindgen::prelude::*;

// #[cfg_attr(
// 	feature = "web-not-node",
// 	wasm_bindgen(module = "/target/js/bundle-es.js")
// )]
// #[cfg_attr(
// 	feature = "node-not-web",
// 	wasm_bindgen(module = "/target/js/bundle-cjs.js")
// )]
// extern "C" {
// 	#[allow(non_camel_case_types)]
// 	#[wasm_bindgen(js_name = "app")]
// 	type _app;

// 	/// Takes a config object and returns a firebase app instance
// 	///
// 	/// Equivalent to:
// 	/// ```js
// 	/// import { initializeApp } from 'firebase/app';
// 	///
// 	/// // Get your own config from somewhere, typically copy-paste from firebase console
// 	/// const config = {
// 	/// 	apiKey: "...",
// 	/// 	projectId: "...",
// 	/// 	...
// 	/// }
// 	///
// 	/// initializeApp(config);
// 	/// ```
// 	///
// 	#[wasm_bindgen(catch, static_method_of = _app, js_name = "initializeApp")]
// 	pub fn initialize_app(config: &JsValue, name: Option<String>) -> Result<JsValue, JsValue>;
// }

// pub fn initialize_app(config: &JsValue, name: Option<String>) -> Result<JsValue, JsValue> {
// 	cfg_if::cfg_if! {
// 		if #[cfg(feature = "verbose-logging")] {
// 			let target = firebase_js_sys_proc::target_name!();

// 			log::info!("firebase-js-sys({}): firebase/app :: initialize_app({:?}, {:?})", target, config, name);
// 		}
// 	}

// 	_app::initialize_app(config, name)
// }

#[js_bind("app")]
pub fn initialize_app(config: &JsValue, name: Option<String>) -> Result<JsValue, JsValue> {}

pub fn tt() {
	_app::initialize_app(&JsValue::UNDEFINED, None);
	initialize_app(&JsValue::UNDEFINED, None);
}