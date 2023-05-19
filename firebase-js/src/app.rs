use firebase_types::FirebaseConfig;
use wasm_bindgen::{JsCast, JsValue};

use crate::{db::Db, Global};

#[derive(Debug, Clone)]
pub struct App<'g> {
	pub(self) js: JsValue,

	global: &'g Global,
}

impl App {
	pub fn initialize_app(config: FirebaseConfig) -> Result<App, JsValue> {
		let config = serde_wasm_bindgen::to_value(&config)?;
		let config = config.dyn_into()?;

		let app = firebase_js_sys::app::initialize_app(config, None)?;

		Ok(App::from_js(app))
	}

	fn from_js(js: JsValue) -> Self {
		Self { js }
	}

	pub fn get_db(&self) -> Db {
		let db =
			firebase_js_sys::database::get_database(&self.js, None).expect("Failed to get database");
		Db::from_js(db)
	}
}
