use std::rc::Rc;

use firebase_js_sys::{DatabaseSnapshot, database};
use log::info;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{app::FirebaseApp, FirebaseError};

// pub struct FirebaseDatabase<'a>(&'a JsValue);
pub struct Database(JsValue);
// pub struct FirebaseDbReference<'a>(&'a JsValue);
pub struct DatabaseReference(JsValue);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_db_ref_from() {
		let db_ref = DatabaseReference(JsValue::from_str("test"));

		let closure = move | snapshot: Result<String, _> | {
			let str: String = snapshot.ok().unwrap();
		};

		on_value_changed(&db_ref, Box::new(closure));
	}

	#[test]
	fn test_on_value_changed() {
		
	}
}

/// Takes a [FirebaseApp] instance and a [url] and returns a [FirebaseDatabase] instance.
/// Fails only if underlying JS function fails.
///
/// ## Examples
/// ```rs
/// use firebase_js_sys::{app::initialize_app, database::get_database};
///
/// let app = initialize_app(config); // config is a FirebaseConfig instance
/// let db = get_database(&app, "https://my-project.firebaseio.com");
/// ```
pub fn get_database<'a>(app: &'a FirebaseApp, url: String) -> Result<Database, FirebaseError> {
	let database = database::get_database_from_url(app.get_js_value(), url);
	Ok(Database(database))
}

/// Takes a [FirebaseDatabase] instance and a [path] and returns a [FirebaseDbReference] instance.
/// Fails only if underlying JS function fails.
///
/// You can think of the returne [FirebaseDbReference] as a pointer into a specific part of your database,
/// which you can use in conjunction with other functions to read and write data.
///
/// ## Examples
/// ```rs
/// use firebase_js_sys::{app::initialize_app, database::{get_database, get_ref}};
///
/// let app = initialize_app(config); // config is a FirebaseConfig instance
/// let db = get_database(&app, "https://my-project.firebaseio.com");
///
/// let db_ref = get_ref(&db, "users/1234");
/// let db_ref2 = get_ref(&db, "users/1234/name");
/// ```
pub fn get_ref(db: &Database, path: String) -> Result<DatabaseReference, FirebaseError> {
	let reference: JsValue = database::get_ref(&db.0, path);
	Ok(DatabaseReference(reference))
}

/// See [get_ref] documentation. Basically gains a reference to the root of your database,
/// like (but not equivalent I don't think) to calling [get_ref] with a path of `""` or `"/"`.
///
/// ## Examples
/// ```rs
/// use firebase_js_sys::database::get_ref_of_root;
///
/// let db_ref = get_ref_of_root(&db, "/");
/// ```
pub fn get_ref_of_root(db: &Database) -> Result<DatabaseReference, FirebaseError> {
	let reference: JsValue = database::get_ref_no_path(&db.0);
	Ok(DatabaseReference(reference))
}

/// Registers a [callback] to be executed every time some data at the specified [DatabaseReference] changes.
/// Note: This closure will be called the first time the data becomes available.
///
/// ## Examples
/// ```rs
/// use firebase_js::database::{get_database, get_ref, on_value_changed};
///
/// let db = get_database(&app, "https://my-project.firebaseio.com");
/// let db_ref = get_ref(&db, "users/1234");
///
/// on_value_changed(&db_ref, |data| {
/// 	// do something with data
/// });
/// ```
///
/// ## WIP:
/// - Unsubscribing does not work yet
/// - Potential for convenienve func to take [String] instead of &[DatabaseReference]
pub fn on_value_changed<T>(
	db_location_reference: &DatabaseReference,
	callback: Box<dyn Fn(Result<T, serde_wasm_bindgen::Error>)>,
)
where
	T: serde::de::DeserializeOwned + 'static,
{
	let raw_closure: Closure<(dyn FnMut(DatabaseSnapshot) + 'static)> = Closure::wrap(Box::new(move | snapshot: DatabaseSnapshot | {
		let values: JsValue = snapshot.values();
		info!("firebase-js: on_value_changed callback fired with {:?}", values.clone());
		// let err_msg = format!("Could not deserialize: {:?}", raw_obj.clone());
		let data = serde_wasm_bindgen::from_value(values.clone());

		callback(data)
	}));

	// TODO: implement unsubscribe, I've not needed it yet
	#[allow(unused_variables)]
	let unsubscribe = database::on_value(&db_location_reference.0, &raw_closure);

	raw_closure.forget();
}