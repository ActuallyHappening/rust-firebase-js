use firebase_js_sys::app::initialize_app;
use js_sys::Object;
use log::info;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("Running ...");

	let _app = initialize_app(Object::new(), None).expect("didn't err?");
}