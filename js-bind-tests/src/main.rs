use js_bind::js_bind;

fn main() {
	// testing

	#[js_bind(method = "top-level-function")]
	pub fn test() {}

	#[js_bind(method = "top-level-function")]
	pub fn test_again() {}

	// println!("Works?: {}", works());
}