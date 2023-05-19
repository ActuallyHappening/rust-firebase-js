use std::marker::PhantomData;

use firebase_js_sys::database::DatabaseSnapshot;
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{FResult, ClosureGlobal, TClosure};

pub struct Db {
	pub(crate) js: JsValue,

	global: &'static ClosureGlobal<DatabaseSnapshot>,
}

impl Db {
	pub(crate) fn from_js(global: &'static ClosureGlobal<DatabaseSnapshot>, js: JsValue) -> Self {
		Self { js, global }
	}

	pub fn get_ref(&self, path: &str) -> FResult<DbRef> {
		let ref_ = firebase_js_sys::database::get_ref(&self.js, Some(path))?;
		Ok(DbRef::from_js(self.global, ref_))
	}
}

pub struct DbRef<'global> {
	pub(crate) js: JsValue,
	global: &'global ClosureGlobal<DatabaseSnapshot>,
}

impl<'g> DbRef<'g> {
	pub(self) fn from_js(global: &'g ClosureGlobal<DatabaseSnapshot>, js: JsValue) -> Self {
		Self { js, global }
	}

	pub fn on_value(&self, callback: impl FnMut(DatabaseSnapshot) + 'static) -> FResult<()> {
		let closure: &TClosure<DatabaseSnapshot> = &Closure::wrap(Box::new(callback));
		firebase_js_sys::database::on_value(&self.js, closure)?;

		// self.global.register_closure(callback);

		// Ok(())
		unimplemented!()
	}
}
