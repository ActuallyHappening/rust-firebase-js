use firebase_js_sys::app::ModuleApp;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
pub struct FirebaseConfig {
	#[serde(rename = "apiKey")]
	pub api_key: String,

	#[serde(rename = "authDomain")]
	pub auth_domain: String,

	#[serde(rename = "projectId")]
	pub project_id: String,

	#[serde(rename = "storageBucket")]
	pub storage_bucket: String,

	#[serde(rename = "messagingSenderId")]
	pub messaging_sender_id: String,

	#[serde(rename = "appId")]
	pub app_id: String,

	#[serde(rename = "measurementId")]
	pub measurement_id: String,

	#[serde(rename = "databaseURL")]
	pub database_url: String,
}

pub struct FirebaseApp(JsValue);

impl Into<JsValue> for FirebaseApp {
	fn into(self) -> JsValue {
		self.0
	}
}

impl FirebaseApp {
	pub fn get_js_value(&self) -> &JsValue {
		&self.0
	}
}

pub fn initialize_app(firebase_config: &FirebaseConfig) -> Result<FirebaseApp, JsValue> {
	let config: JsValue = to_value(firebase_config)?;
	let app: JsValue = ModuleApp::initialize_app(&config);

	Ok(FirebaseApp(app))
}
