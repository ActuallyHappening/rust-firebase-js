use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/firebase-interop/app.js")]
extern {
	pub fn test();
}