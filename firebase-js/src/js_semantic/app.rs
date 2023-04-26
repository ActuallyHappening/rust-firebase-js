use derive_deref_rs::Deref;
use firebase_js_sys::app;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use serde_wasm_bindgen::to_value;

use crate::FirebaseError;

#[derive(Serialize, Deserialize)]
pub struct FirebaseConfig {
	#[serde(rename = "apiKey")]
	pub api_key: Option<String>,

	#[serde(rename = "authDomain")]
	pub auth_domain: Option<String>,

	#[serde(rename = "projectId")]
	pub project_id: Option<String>,

	#[serde(rename = "storageBucket")]
	pub storage_bucket: Option<String>,

	#[serde(rename = "messagingSenderId")]
	pub messaging_sender_id: Option<String>,

	#[serde(rename = "appId")]
	pub app_id: Option<String>,

	#[serde(rename = "measurementId")]
	pub measurement_id: Option<String>,

	#[serde(rename = "databaseURL")]
	pub database_url: Option<String>,
}

#[derive(Deref)]
pub struct FirebaseApp(JsValue);

pub fn initialize_app(firebase_config: &FirebaseConfig) -> Result<FirebaseApp, FirebaseError> {
	match to_value(firebase_config) {
		Ok(val) => {
			let app: JsValue = app::initialize_app(&val);
			Ok(FirebaseApp(app))
		},
		Err(_) => Err(FirebaseError::UnimplementedErrorHandling),
	}
}
