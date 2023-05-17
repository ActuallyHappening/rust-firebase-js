use js_sys::JsString;
use wasm_bindgen::{JsValue, JsCast};

/// Returns every equivalent representation of a JS string.
pub fn get_strings(str: &str) -> Vec<JsValue> {
	vec! [
		JsValue::from_str(str),
		JsValue::from_str(str).as_string().unwrap().into(),
		JsValue::from_str(str).dyn_ref::<JsValue>().unwrap().into(),
		JsValue::from_str(str).dyn_ref::<JsString>().unwrap().into(),

		JsString::from(str).into(),
	]
}

/// Returns every equivalent representation of a JS bool.
pub fn get_bools(bool: bool) -> Vec<JsValue> {
	vec! [
		JsValue::from_bool(bool),
		JsValue::from_bool(bool).as_string().unwrap().into(),
		JsValue::from_bool(bool).dyn_ref::<JsValue>().unwrap().into(),
		JsValue::from_bool(bool).dyn_ref::<JsString>().unwrap().into(),

		match bool {
			true => JsValue::TRUE,
			false => JsValue::FALSE,
		}
	]
}