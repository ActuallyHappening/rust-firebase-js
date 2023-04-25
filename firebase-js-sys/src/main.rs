use log::info;
use wasm_bindgen::JsValue;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("YES!");
	
	let returned = firebase_js_sys::ModuleApp::initialize_app(&JsValue::UNDEFINED);
	// println!("returned: {:?}", returned);
}