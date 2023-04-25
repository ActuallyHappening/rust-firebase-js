use firebase_js_sys::database::ModuleDatabase;
use wasm_bindgen::JsValue;

use crate::{app::FirebaseApp, closure};

pub struct FirebaseDatabase(JsValue);

impl Into<JsValue> for FirebaseDatabase {
	fn into(self) -> JsValue {
		self.0
	}
}

impl FirebaseDatabase {
	pub fn get_js_value(&self) -> &JsValue {
		&self.0
	}
}

pub fn get_database(app: &FirebaseApp, url: String) -> Result<FirebaseDatabase, JsValue> {
	let database: JsValue = ModuleDatabase::get_database_from_url(app.get_js_value(), url);
	Ok(FirebaseDatabase(database))
}

pub fn get_ref(database: &FirebaseDatabase, path: String) -> Result<JsValue, JsValue> {
	let reference: JsValue = ModuleDatabase::get_ref(database.get_js_value(), path);
	Ok(reference)
}

pub fn on_value_changed(reference: &FirebaseDatabase, callback: &closure<JsValue>) {
	// TODO: implement unsubscribe, I've not needed it yet
	#[allow(unused_variables)]
	let unsubscribe = ModuleDatabase::on_value(reference.get_js_value(), callback);
}

