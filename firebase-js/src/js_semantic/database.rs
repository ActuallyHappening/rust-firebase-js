use std::ops::Deref;

use derive_deref_rs::Deref;
use firebase_js_sys::{database, DatabaseSnapshot};
use log::info;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{js_semantic::app::FirebaseApp, FirebaseError};

#[derive(Deref)]
pub struct Database(JsValue);

#[derive(Deref)]
pub struct DatabaseReference(JsValue);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_db_ref_from() {
		let db_ref = DatabaseReference(JsValue::from_str("test"));

		let closure = move |snapshot: Result<String, _>| {
			let str: String = snapshot.ok().unwrap();
		};

		on_value_changed(&db_ref, Box::new(closure));
	}

	#[test]
	fn test_on_value_changed() {}
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
pub fn get_database(app: &FirebaseApp, url: String) -> Result<Database, FirebaseError> {
	let app: &JsValue = app.deref();
	let database = database::get_database_from_url(app, url);
	Ok(Database(database))
}

/// Takes a [FirebaseDatabase] instance and a [path] and returns a [FirebaseDbReference] instance.
/// Fails only if underlying JS function fails.
///
/// You can think of the returne [FirebaseDbReference] as a pointer into a specific part of your database,
/// which you can use in conjunction with other functions to read and write data.
/// 
/// Passing [None] to path will return a reference to the root of your database.
///
/// ## Examples
/// ```rs
/// use firebase_js_sys::{app::initialize_app, database::{get_database, get_ref}};
///
/// let app = initialize_app(config); // config is a FirebaseConfig instance
/// let db = get_database(&app, "https://my-project.firebaseio.com");
///
/// let db_ref = get_ref(&db, Some("users/1234"));
/// let db_ref2 = get_ref(&db, Some("users/1234/name"));
/// ```
pub fn get_ref(db: &Database, path: Option<String>) -> Result<DatabaseReference, FirebaseError> {
	let reference = match path {
		Some(path) => {
			database::get_ref(&db.0, path)
		}
		None => {
			database::get_ref_no_path(&db.0)
		}
	};
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
pub fn on_value_changed<T>(
	db_location_reference: &DatabaseReference,
	callback: Box<dyn Fn(Result<T, serde_wasm_bindgen::Error>)>,
) where
	T: serde::de::DeserializeOwned + 'static,
{
	let raw_closure: Closure<(dyn FnMut(DatabaseSnapshot) + 'static)> =
		Closure::wrap(Box::new(move |snapshot: DatabaseSnapshot| {
			let values: JsValue = snapshot.values();

			// let err_msg = format!("Could not deserialize: {:?}", raw_obj.clone());
			let data = serde_wasm_bindgen::from_value(values.clone());

			match data {
				Err(_) => {
					info!(
						"firebase-js: on_value_changed callback failed to deserialize data: {:?}",
						values.clone()
					);
				}

				_ => {
					info!(
						"firebase-js: on_value_changed callback fired with {:?}",
						values.clone()
					)
				}
			}

			callback(data)
		}));

	// TODO: implement unsubscribe, I've not needed it yet
	let unsubscribe = database::on_value(&db_location_reference.0, &raw_closure);

	// TODO: Find a better solution than leaking memory!
	raw_closure.forget();

	// unsubscribe
}
