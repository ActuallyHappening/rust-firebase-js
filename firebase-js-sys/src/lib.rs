//! `firebase_js_sys` is a wrapper around the firebase javascript sdk, allowing you to use it in rust.
//! 
//! See [this package's README](https://github.com/ActuallyHappening/rust-firebase-js/blob/master/firebase-js-sys/README.md) for more information,
//! and [the `firebase-js` package](https://github.com/ActuallyHappening/rust-firebase-js/tree/master/firebase-js) for a high level implementation
//! layer ontop of this crate.
//! 
//! Don't use this crate unless you are prepared to implement a lot more conversion logic, as most
//! of the functions return `JsValue`s instead of a more useful types.
//! See crate `firebase-js` for a more ergonomic interface.
//! 
//! Uses bindings hand-written using `#[wasm_bindgen]` and `rollup`.
//! 
//! ## Examples:
//! Usage in general:
//! ```rs
//! use firebase_js_sys::ModuleApp;
//! 
//! // Will give runtime console error
//! ModuleApp::initialize_app(&JsValue::UNDEFINED);
//! ```
//! 
//! Example main.rs for using `trunk` to build + run in browser:
//! ```rs
//! use log::info;
//! use wasm_bindgen::JsValue;
//! 
//! fn main() {
//! 	_ = console_log::init_with_level(log::Level::Debug);
//! 	
//! 	console_error_panic_hook::set_once();
//! 
//! 	info!("main.rs is running!");
//! 	
//! 	// Will not work, but only gives console run time error
//! 	let app = firebase_js_sys::ModuleApp::initialize_app(&JsValue::UNDEFINED);
//! 	// println!("returned: {:?}", app);
//! }
//! ```


use wasm_bindgen::prelude::*;

type closure<Args> = Closure<dyn FnMut(Args)>;

pub use app::ModuleApp;
pub use database::ModuleDatabase;

mod app {
	use wasm_bindgen::prelude::*;

	#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
	extern "C" {
		pub type ModuleApp;

		/// Takes a config object and returns a firebase app instance
		/// 
		/// Equivalent to:
		/// ```js
		/// import { initializeApp } from 'firebase/app';
		/// 
		/// // Get your own config from somewhere, typically copy-paste from firebase console
		/// const config = {
		/// 	apiKey: "...",
		/// 	projectId: "...",
		/// 	...
		/// }
		/// 
		/// initializeApp(config);
		/// ```
		#[wasm_bindgen(static_method_of = ModuleApp, js_name = "initializeApp")]
		pub fn initialize_app(config: &JsValue) -> JsValue;
	}
}

mod database {
	use wasm_bindgen::prelude::*;
	use crate::closure;

	#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
	extern "C" {
		pub type ModuleDatabase;

		/// Takes a firebase app instance (reference) and returns a reference to the database associated with that app
		/// 
		/// Equivalent to:
		/// ```js
		/// import { getDatabase } from 'firebase/database';
		/// 
		/// const app = initializeApp(config); // Get your own config from somewhere, typically copy-paste from firebase console
		/// 
		/// const db = getDatabase(app);
		/// ```
		#[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "getDatabase")]
		pub fn get_database_from_url(db: &JsValue, url: String) -> JsValue;

		/// Takes a database instance (reference) and returns a firebase reference to the database, representing a specific path of the database
		/// You can use this in other functions, e.g. `on_value(db_ref:)`'s first argument (`db_ref`) is returned by this function
		/// 
		/// Equivalent to:
		/// ```js
		/// import { ref } from 'firebase/database';
		/// 
		/// const db = getDatabase(app); // Get your own app from somewhere
		/// 
		/// ref(db, path);
		/// ```
		/// 
		/// See [get_ref_no_path] for usage without a `path` argument
		#[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "ref")]
		pub fn get_ref(db: &JsValue, path: String) -> JsValue;

		/// See `get_ref`'s documentation, returns a reference to the root of the database
		/// 
		/// Equivalent to:
		/// ```js
		/// import { ref } from 'firebase/database';
		/// 
		/// const db = getDatabase(app); // Get your own app from somewhere
		/// 
		/// ref(db);
		/// // Note how is it NOT equivalent to:
		/// // ref(db, "")
		/// ```
		#[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "ref")]
		pub fn get_ref_no_path(db: &JsValue) -> JsValue;

		// #[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "getDatabase")]
		// pub fn get_default_database(db: &JsValue) -> JsValue;

		/// Registers a callback to be called when the data at the specified path changes
		/// 
		/// Equivalent to:
		/// ```js
		/// import { onValue } from 'firebase/database';
		/// ```
		#[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "onValue")]
		pub fn on_value(db_ref: &JsValue, callback: &closure<JsValue>) -> JsValue;
	}
}

// #[wasm_bindgen(module = "/firebase-interop/database.js")]
// extern {

// 	#[wasm_bindgen(js_name = "ref")]
// 	pub fn get_ref(db: &JsValue, path: String) -> JsValue;

// }
