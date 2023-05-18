use wasm_bindgen::JsValue;


#[derive(Debug, Clone)]
pub struct App {
	pub(crate) js: JsValue,
}

#[derive(Debug, Clone)]
pub struct Config {
	
}

impl App {
	pub fn initialize_app(config: Config)
}