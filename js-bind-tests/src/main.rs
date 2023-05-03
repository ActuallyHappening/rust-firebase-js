use js_bind::js_bind;
// use wasm_bindgen::prelude::wasm_bindgen;
use js_bind::Config;

#[js_bind(js_module = "test/app")]
extern "C" {
	// #[wasm_bindgen]
	/// Documentation!
	pub fn works() -> bool;
}

fn main() {
	// let string =
	// 	std::fs::read_to_string("./js-bind.toml".to_string()).expect("Couldn't read file");
	// let config = toml::from_str::<Config>(string.as_str());
	// match config {
	// 	Ok(config) => {
	// 		// println!("Config: {:?}", config);
	// 	},
	// 	Err(err) => {
	// 		// println!("Error: {:?}", err);
	// 		// panic
	// 		panic!("Error: {:?}", err);
	// 	},
	// }

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
