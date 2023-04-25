use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/firebase-interop/app.js")]
extern {
	#[wasm_bindgen(js_name = "initializeApp")]
	pub fn initialize_app(config: &JsValue) -> JsValue;
}