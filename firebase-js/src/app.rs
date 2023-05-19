use firebase_types::FirebaseConfig;
use wasm_bindgen::{JsValue, JsCast};


#[derive(Debug, Clone)]
pub struct App {
	pub(crate) js: JsValue,
}

impl App {
	pub fn initialize_app(config: FirebaseConfig) -> Result<App, JsValue> {
		let config = serde_wasm_bindgen::to_value(&config)?;
		let config = config.dyn_into()?;

		let app = firebase_js_sys::app::initialize_app(config, None)?;

		Ok(App { js: app })
	}
}