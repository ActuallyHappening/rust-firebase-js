use firebase_js::app::initialize_app;
use log::info;

mod secrets;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("firebase-js: main.rs()");
	
	let returned = initialize_app(&secrets::config);
	// println!("returned: {:?}", returned);
}