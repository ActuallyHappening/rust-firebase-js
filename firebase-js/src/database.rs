use std::ops::Deref;

use firebase_js_sys::database::ModuleDatabase;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

use crate::{app::FirebaseApp, closure};

// pub struct FirebaseDatabase<'a>(&'a JsValue);
pub struct FirebaseDatabase(JsValue);

// impl<'a> AsRef<&JsValue> for FirebaseDatabase<'a> {
// 	fn as_ref(&self) -> &JsValue {
// 		&self.0
// 	}
// }

// impl<'a> From<FirebaseDatabase<'a>> for &JsValue {
// 	fn from(db: FirebaseDatabase) -> Self {
// 		db.0
// 	}
// }

// impl<'a> JsCast for FirebaseDatabase<'a> {
// 	fn instanceof(val: &JsValue) -> bool {
// 		ModuleDatabase::instanceof(val)
// 	}

// 	fn unchecked_from_js(val: JsValue) -> Self {
// 		FirebaseDatabase(&val)
// 	}

// 	fn unchecked_from_js_ref(val: &JsValue) -> &Self {
// 		&FirebaseDatabase(val)
// 	}
// }

// pub struct FirebaseDbReference<'a>(&'a JsValue);
pub struct FirebaseDbReference(JsValue);

// impl<'a> AsRef<JsValue> for FirebaseDbReference<'a> {
// 	fn as_ref(&self) -> &JsValue {
// 		&self.0
// 	}
// }

// impl<'a> From<FirebaseDbReference<'a>> for JsValue {
// 	fn from(db: FirebaseDbReference) -> Self {
// 		*db.0
// 	}
// }

// impl<'a> JsCast for FirebaseDbReference<'a> {
// 	fn instanceof(val: &JsValue) -> bool {
// 		ModuleDatabase::instanceof(val)
// 	}

// 	fn unchecked_from_js(val: JsValue) -> Self {
// 		FirebaseDbReference(&val)
// 	}

// 	fn unchecked_from_js_ref(val: &JsValue) -> &Self {
// 		&FirebaseDbReference(val)
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_db_ref_from() {
		let db_ref = FirebaseDatabase(JsValue::from_str("test"));
		let js_obj: JsValue = db_ref.0;
	}
}

pub fn get_database<'a>(app: &'a FirebaseApp, url: String) -> Result<FirebaseDatabase, JsValue> {
	let database = ModuleDatabase::get_database_from_url(app.get_js_value(), url);
	Ok(FirebaseDatabase(database))
}

pub fn get_ref(db: &FirebaseDatabase, path: String) -> Result<FirebaseDbReference, JsValue> {
	let reference: JsValue = ModuleDatabase::get_ref(&db.0, path);
	Ok(FirebaseDbReference(reference))
}

pub fn get_ref_of_root(db: &FirebaseDatabase) -> Result<FirebaseDbReference, JsValue> {
	let reference: JsValue = ModuleDatabase::get_ref_no_path(&db.0);
	Ok(FirebaseDbReference(reference))
}

pub fn on_value_changed(db: &FirebaseDbReference, callback: &'static dyn Fn(JsValue)) {
	let closure: closure<JsValue> = Closure::wrap(Box::new(callback));

	// TODO: implement unsubscribe, I've not needed it yet
	#[allow(unused_variables)]
	let unsubscribe = ModuleDatabase::on_value(&db.0, &closure);
}
