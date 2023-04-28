// use log::info;
// use wasm_bindgen::JsValue;

// fn main() {
// 	_ = console_log::init_with_level(log::Level::Debug);
// 	console_error_panic_hook::set_once();

// 	info!("YES!");
	
// 	let _returned = firebase_js_sys::app::initialize_app(&JsValue::UNDEFINED, None);
// 	// println!("returned: {:?}", returned);
// }

use firebase_js_sys_proc::duplicate_test;

#[duplicate_test]
fn manual_initialize_app_empty() {
	let result = app::initialize_app(&JsValue::UNDEFINED, None);
	
	assert!(result.is_err());
	// panic!("Error description: {:?}", result.err().unwrap());
}