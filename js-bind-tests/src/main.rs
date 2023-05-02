use js_bind::js_bind;
// use wasm_bindgen::prelude::wasm_bindgen;

#[js_bind(module = "test/app")]
extern "C" {
	// #[wasm_bindgen]
	/// Documentation!
	#[js_bind(doc, test)]
	pub fn works() -> bool;

}



fn main() {
	// works();
	// testing

	// Documentation!
	// #[js_bind]
	// pub fn test() {}

	// #[js_bind(method = "top-level")]
	// pub fn test() {}

	// #[js_bind(method = "top-level")]
	// pub fn test_again() {}

	// println!("W∏orks?: {}", works());
}