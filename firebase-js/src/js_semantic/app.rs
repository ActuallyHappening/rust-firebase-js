use derive_deref_rs::Deref;
use firebase_js_sys::app;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use derive_new::new;

use crate::FirebaseError;

#[derive(Serialize, Deserialize, new)]
pub struct FirebaseConfig {
	#[serde(rename = "projectId")]
	pub project_id: String,

	#[new(default)]
	#[serde(rename = "apiKey")]
	pub api_key: Option<String>,

	#[new(default)]
	#[serde(rename = "authDomain")]
	pub auth_domain: Option<String>,

	#[new(default)]
	#[serde(rename = "storageBucket")]
	pub storage_bucket: Option<String>,

	#[new(default)]
	#[serde(rename = "messagingSenderId")]
	pub messaging_sender_id: Option<String>,

	#[new(default)]
	#[serde(rename = "appId")]
	pub app_id: Option<String>,

	#[new(default)]
	#[serde(rename = "measurementId")]
	pub measurement_id: Option<String>,

	#[new(default)]
	#[serde(rename = "databaseURL")]
	pub database_url: Option<String>,
}

#[derive(Deref)]
pub struct FirebaseApp(JsValue);
