use wasm_bindgen::prelude::*;

type closure<Args> = Closure<dyn FnMut(Args)>;

pub mod app {
  use wasm_bindgen::prelude::*;

	#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
	extern "C" {
		pub type ModuleApp;

		#[wasm_bindgen(static_method_of = ModuleApp, js_name = "initializeApp")]
		pub fn initialize_app(config: &JsValue) -> JsValue;
	}
}

pub mod database {
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
	extern "C" {
		pub type ModuleDatabase;

		#[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "ref")]
		pub fn get_ref(config: &JsValue) -> JsValue;
	}
}


// #[wasm_bindgen(module = "/firebase-interop/database.js")]
// extern {
// 	#[wasm_bindgen(js_name = "getDatabase")]
// 	pub fn get_database_from_url(db: &JsValue, url: String) -> JsValue;

// 	#[wasm_bindgen(js_name = "getDatabase")]
// 	pub fn get_default_database(db: &JsValue) -> JsValue;

// 	#[wasm_bindgen(js_name = "ref")]
// 	pub fn get_ref(db: &JsValue, path: String) -> JsValue;

// 	#[wasm_bindgen(js_name = "onValue")]
// 	pub fn on_value(db_ref: &JsValue, callback: &closure<JsValue>) -> JsValue;
// }