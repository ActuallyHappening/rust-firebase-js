use crate::closure;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/firebase-interop/bundle.js")]
extern "C" {
	#[allow(non_camel_case_types)]
	pub type database;

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
	#[wasm_bindgen(static_method_of = database, js_name = "getDatabase")]
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
	#[wasm_bindgen(static_method_of = database, js_name = "ref")]
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
	#[wasm_bindgen(static_method_of = database, js_name = "ref")]
	pub fn get_ref_no_path(db: &JsValue) -> JsValue;

	/// Represents a snapshow of the firebase database,
	/// get the actual values using `snapshot.values()`
	pub type DatabaseSnapshot;

	/// Returns the value of a [DatabaseSnapshot]
	#[wasm_bindgen(method, js_name = "val")]
	pub fn values(this: &DatabaseSnapshot) -> JsValue;

	// #[wasm_bindgen(static_method_of = ModuleDatabase, js_name = "getDatabase")]
	// pub fn get_default_database(db: &JsValue) -> JsValue;

	/// Registers a callback to be called when the data at the specified path changes.
	/// Returns a database snapshot, which you can call `.val()` to receive values.
	///
	/// Equivalent to:
	/// ```js
	/// import { onValue } from 'firebase/database';
	/// ```
	#[wasm_bindgen(static_method_of = database, js_name = "onValue")]
	pub fn on_value(db_ref: &JsValue, callback: &closure<DatabaseSnapshot>) -> JsValue;
}
