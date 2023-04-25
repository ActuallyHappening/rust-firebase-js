use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::closure;

#[wasm_bindgen(module = "/firebase-interop/database.js")]
extern {
	#[wasm_bindgen(js_name = "getDatabase")]
	pub fn get_database_from_url(db: &JsValue, url: String) -> JsValue;

	#[wasm_bindgen(js_name = "getDatabase")]
	pub fn get_default_database(db: &JsValue) -> JsValue;

	#[wasm_bindgen(js_name = "ref")]
	pub fn get_ref(db: &JsValue, path: String) -> JsValue;

	#[wasm_bindgen(js_name = "onValue")]
	pub fn on_value(db_ref: &JsValue, callback: &closure<JsValue>) -> JsValue;
}