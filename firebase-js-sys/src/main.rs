#![allow(unused_imports)]

use firebase_js_sys::app::initialize_app;
use js_sys::{Reflect, Object};
use log::info;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str, obj: &JsValue);

	#[wasm_bindgen(js_namespace = console)]
	fn warn(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("firebase-js: main.rs()");

	let _err = initialize_app(Object::new(), None).expect("Couldn't init app");

	// log("err: ", &err);

	// let code1 = Reflect::get(&err, &JsValue::from_str("code")).expect("Failed to get code");
	// log("code1: ", &code1);
}