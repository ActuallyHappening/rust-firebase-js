use firebase_js_sys::database::DatabaseSnapshot;
use firebase_types::FirebaseConfig;
use wasm_bindgen::{JsCast, JsValue};

use crate::{db::Db, ClosureGlobal, FResult};

#[derive(Debug, Clone)]
pub struct App {
	pub(self) js: JsValue,

	global: &'static ClosureGlobal<DatabaseSnapshot>,
}

impl App {
	pub fn initialize_app(global: &'static ClosureGlobal<DatabaseSnapshot>, config: FirebaseConfig) -> Result<App, JsValue> {
		let config = serde_wasm_bindgen::to_value(&config)?;
		let config = config.dyn_into()?;

		let app = firebase_js_sys::app::initialize_app(config, None)?;

		Ok(App::from_js(global, app))
	}

	fn from_js(global: &'static ClosureGlobal<DatabaseSnapshot>, js: JsValue) -> Self {
		Self { js, global }
	}

	pub fn get_db(&self) -> FResult<Db> {
		let db =
			firebase_js_sys::database::get_database(&self.js, None)?;
		Ok(Db::from_js(&self.global, db))
	}

	pub fn get_db_with_url(&self, url: &str) -> Db {
		let db = firebase_js_sys::database::get_database(&self.js, Some(url))
			.expect("Failed to get database");
		Db::from_js(&self.global, db)
	}
}
