use firebase_js_sys::app;
use wasm_bindgen::JsValue;

fn main() {
	let result = app::initialize_app(JsValue::UNDEFINED);
	
	assert!(result.is_err());
	panic!("Error description: {:?}", result.err().unwrap());
}