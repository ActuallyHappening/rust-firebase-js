use firebase_js::{app::initialize_app, database::get_database};
use log::info;

use crate::secrets::URL;

mod secrets;

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("firebase-js: main.rs()");
	
	let app = initialize_app(&secrets::get_config()).ok().unwrap();

	let db = get_database(&app, URL.to_string());
}