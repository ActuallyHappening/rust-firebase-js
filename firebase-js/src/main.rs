use firebase_js::{app::initialize_app, database::{get_database, on_value_changed, get_ref_of_root, get_ref}};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::secrets::URL;

mod secrets;

#[derive(Debug, Deserialize, Serialize)]
struct Test {
	test: String
}

#[cfg(test)]
mod tests {
    use crate::Test;

	#[test]
	fn test_de_owned() {
	}
}

// impl serde::de::DeserializeOwned for Test {}

// Add console.log binding
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str, obj: &JsValue);

	#[wasm_bindgen(js_namespace = console)]
	fn warn(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
}

fn main() {
	_ = console_log::init_with_level(log::Level::Debug);
	console_error_panic_hook::set_once();

	info!("firebase-js: main.rs()");
	
	let app = initialize_app(&secrets::get_config()).ok().unwrap();

	let db = get_database(&app, URL.to_string()).ok().unwrap();

	// let reference = get_ref_of_root(&db).ok().expect("Failed to get root reference");
	let reference = get_ref(&db, "/test".to_string()).ok().unwrap();

	let test = serde_wasm_bindgen::to_value(&Test { test: "test123".to_string() }).ok().unwrap();
	log("test: {:?}", &test);
	info!("test: {:?}", test);
	let test_de: Test = serde_wasm_bindgen::from_value(test).ok().unwrap();
	info!("test_de: {:?}", test_de);

	let closure = on_value_changed(&reference, &move |event, raw_obj| {
		info!("RS: raw_obj: {:?}", raw_obj);
		log("RS: raw_obj", &raw_obj);
		let e: Test = event.ok().unwrap();
		info!("RS: on_value_changed() WOW! {:?}", e)
	});

	// closure.forget();
}