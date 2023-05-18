#![allow(unused_imports)]

use extract_doctests::extract_doctests;
use js_sys::{Error, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test;

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
	/// ```rust,no_run
	/// // extract-doctests initialize_app
	/// use firebase_js_sys::app::initialize_app;
	/// use wasm_bindgen::JsValue;
	/// use js_sys::{Object, Reflect};
	///
	/// fn set<T>(target: &JsValue, prop: &str, val: T) where JsValue: From<T> {
	/// 	Reflect::set(&target, &JsValue::from_str(prop), &JsValue::from(val)).unwrap();
	/// }
	/// 
	/// let config = Object::new();
	/// set(&config, "projectId", "test");
	/// 
	/// // initialize with no name
	/// initialize_app(config.clone(), None).expect("Failed to initialize app");
	/// 
	/// // initialize with name
	/// initialize_app(config.clone(), Some("test")).expect("Failed to initialize app");
	/// ```
	#[wasm_bindgen(js_name = "initializeApp", catch)]
	pub fn initialize_app(config: Object, optional_name: Option<&str>) -> Result<JsValue, Error>;
}

// #[cfg(test)]
// #[wasm_bindgen_test]
// fn test_initialize_app() {
// 	#[cfg(feature = "web-not-node")]
// 	::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// 	fn set<T>(target: &JsValue, prop: &str, val: T)
// 	where
// 		JsValue: From<T>,
// 	{
// 		Reflect::set(&target, &JsValue::from_str(prop), &JsValue::from(val)).unwrap();
// 	}

// 	let good_names = vec![None, Some("hello"), Some("📖 unicode supported"),Some("\n newlines"), Some("\t tabs")];

// 	let bad_names = vec![Some(""), ];

// 	let good_configs = vec![
// 		Object::new(),
// 		{
// 			let obj = Object::new();
// 			set(&obj, "projectId", "test");
// 			obj
// 		},
// 		{
// 			let obj = Object::new();
// 			set(&obj, "foo", "test");
// 			set(&obj, "bar", 69);
// 			obj
// 		},
// 		{
// 			let obj = Object::new();
// 			set(&obj, "projectId", "");
// 			obj
// 		},
// 	];

// 	let bad_configs: Vec<Object> = vec![];

// 	fn assert(config: Object, name: Option<&str>, should_err: bool) {
// 		let result = initialize_app(config.clone(), name);
// 		assert!(
// 			match should_err {
// 				true => result.is_err(),
// 				false => result.is_ok(),
// 			},
// 			"Assertion failed: Expected an {} return, found opposite. config: {:?}, name: {:?}. Full Error: {:?}",
// 			match should_err {
// 				true => "error",
// 				false => "ok",
// 			},
// 			config.clone(),
// 			name,
// 			result
// 		);
// 	}

// 	// Should error
// 	bad_configs.clone().into_iter().for_each(|config| {
// 		good_names.clone().into_iter().for_each(|name| {
// 			assert(config.clone(), name, true);
// 		});
// 		bad_names.clone().into_iter().for_each(|name| {
// 			assert(config.clone(), name, true);
// 		});
// 	});

// 	// Shouldn't error
// 	good_configs.clone().into_iter().for_each(|config| {
// 		good_names.clone().into_iter().for_each(|name| {
// 			assert(config.clone(), name, false);
// 		});
// 		bad_names.clone().into_iter().for_each(|name| {
// 			assert(config.clone(), name, true);
// 		});
// 	});
// }
