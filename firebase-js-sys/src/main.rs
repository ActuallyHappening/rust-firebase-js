use std::assert_eq;

use firebase_js_sys::initialize_app;
use js_sys::Reflect;
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

	let err = initialize_app(JsValue::UNDEFINED, JsValue::UNDEFINED).expect_err("didn't err?");

	log("err: ", &err);

	let code1 = Reflect::get(&err, &JsValue::from_str("code")).expect("Failed to get code");
	log("code1: ", &code1);
}