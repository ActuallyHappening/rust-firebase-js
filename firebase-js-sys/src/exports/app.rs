use extract_doctests::extract_doctests;
use js_sys::Error;
use js_sys::Object;
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
	/// // ext ract-doctests initialize_app
	/// use firebase_js_sys::app::initialize_app;
	/// use wasm_bindgen::JsValue;
	/// use js_sys::{Object, Reflect};
	/// 
	/// fn set<T>(target: &JsValue, prop: &str, val: T) where JsValue: From<T> {
	/// 	Reflect::set(&target, &JsValue::from_str(prop), &JsValue::from(val)).unwrap();
	/// }
	/// 
	/// let names = vec![
	/// 	None,
	/// 	Some(""),
	/// 	Some("hello"),
	///   Some("📖 unicode supported"),
	/// ];
	/// 
	/// let valid_configs = vec![
	/// 	Object::new(),
	/// 	{
	/// 		let obj = Object::new();
	/// 		set(&obj, "projectId", "test");
	/// 		obj
	/// 	},
	/// 	{
	/// 		let obj = Object::new();
	/// 		set(&obj, "foo", "test");
	/// 		set(&obj, "bar", 69);
	/// 		obj
	/// 	},
	/// ];
	/// 
	/// let invalid_configs: Vec<Object> = vec![
	/// ];
	/// 
	/// // Should error
	/// for config in invalid_configs.clone() {
	/// 	for name in names.clone() {
	/// 		let result = initialize_app(config.clone(), name);
	/// 		assert!(result.is_err());
	/// 	}
	/// }
	/// 
	/// // Should not error
	/// for config in valid_configs.clone() {
	/// 	for name in names.clone() {
	/// 		let result = initialize_app(config.clone(), name);
	/// 		assert!(result.is_ok());
	/// 	}
	/// }
	/// ```
	#[wasm_bindgen(js_name = "initializeApp", catch)]
	pub fn initialize_app(config: Object, optional_name: Option<&str>) -> Result<JsValue, Error>;
}

// #[cfg(test)]
// #[test]
// fn test_initialize_app() {
//     use js_sys::Reflect;

// 	let config = Object::new();
// 	Reflect::set(&config, &JsValue::from_str("projectId"), &JsValue::from_str("test")).unwrap();

// 	let result = initialize_app(config, None);
// 	assert!(result.is_ok());
// 	let app = result.unwrap();
// 	assert!(app.is_object());
// }
