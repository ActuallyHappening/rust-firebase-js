use js_bind::js_bind;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "js/bundle-es.js")]
extern "C" {
	// #[wasm_bindgen]
	/// Documentation!
	#[js_bind(method="doo")]
	pub fn works() -> bool;
}



fn main() {
	works();
	// testing

	// #[js_bind(method = "top-level")]
	// pub fn test() {}

	// #[js_bind(method = "top-level")]
	// pub fn test_again() {}

	// println!("W∏orks?: {}", works());
}