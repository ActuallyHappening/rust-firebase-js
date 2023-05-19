use extract_doctests::extract_doctests;
use js_sys::Error;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::TClosure;

#[extract_doctests]
#[cfg_attr(feature = "web-not-node", wasm_bindgen(module = "/js/bundle-esm.js"))]
#[cfg_attr(feature = "node-not-web", wasm_bindgen(module = "/js/bundle-cjs.js"))]
extern "C" {
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
	/// 
	/// ## Examples:
	/// Minimal example:
	/// ```rust,no_run
	/// // extract-doctests test_get_database
	/// use firebase_js_sys::app::initialize_app;
	/// use firebase_js_sys::database::get_database;
	/// 
	/// use firebase_js_sys::__testing::get_test_app;
	/// let app = get_test_app();
	/// 
	/// //let app = initialize_app(js_sys::Object::new(), None).expect("Failed to initialize app");
	/// 
	/// let db = get_database(&app, None).expect("Failed to get database");
	/// ```
	#[wasm_bindgen(js_name = "getDatabase", catch)]
	pub fn get_database(app: &JsValue, url: Option<&str>) -> Result<JsValue, Error>;

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
	/// Pass [None] as second arg to get a reference to the root of the database
	/// 
	/// ## Examples:
	/// Minimal example:
	/// ```rust,no_run
	/// // extract-doctests test_get_ref
	/// 
	/// use firebase_js_sys::app::initialize_app;
	/// use firebase_js_sys::database::{get_ref, get_database};
	/// 
	/// use firebase_js_sys::__testing::get_test_app;
	/// let app = get_test_app();
	/// 
	/// let db = get_database(&app, None).expect("Failed to get database");
	/// 
	/// let db_ref = get_ref(&db, Some("test/path")).expect("db ref to be ok");
	/// ```
	#[wasm_bindgen(js_name = "ref", catch)]
	pub fn get_ref(db: &JsValue, path: Option<&str>) -> Result<JsValue, Error>;

	/// Represents a snapshow of the firebase database,
	/// get the actual values using `snapshot.values()`
	pub type DatabaseSnapshot;

	/// Returns the value of a [DatabaseSnapshot]
	#[wasm_bindgen(method, js_name = "val", catch)]
	pub fn values(this: &DatabaseSnapshot) -> Result<JsValue, Error>;

	/// Registers a callback to be called when the data at the specified path changes.
	/// Returns a database snapshot, which you can call `.val()` to receive values.
	///
	/// Equivalent to:
	/// ```js
	/// import { onValue } from 'firebase/database';
	/// ```
	#[wasm_bindgen(js_name = "onValue")]
	pub fn on_value(db_ref: &JsValue, callback: &TClosure<DatabaseSnapshot>) -> JsValue;
}
