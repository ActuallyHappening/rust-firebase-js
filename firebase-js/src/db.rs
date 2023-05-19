use std::marker::PhantomData;

use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{FResult, Global};

pub struct Db<'global> {
	pub(crate) js: JsValue,

	global: &'global Global,
}

impl<'global> Db<'global> {
	pub(crate) fn from_js(global: &'global Global, js: JsValue) -> Self {
		Self { js, global }
	}

	pub fn get_ref(&self, path: &str) -> FResult<DbRef> {
		let ref_ = firebase_js_sys::database::get_ref(&self.js, Some(path))?;
		Ok(DbRef::from_js(&self.global, ref_))
	}
}

pub struct DbRef<'global> {
	pub(crate) js: JsValue,
	global: &'global Global,
}

impl<'g> DbRef<'g> {
	pub(self) fn from_js(global: &'g Global, js: JsValue) -> Self {
		Self { js, global }
	}

	pub fn on_value(&self, callback: impl FnMut(JsValue) + 'static) -> FResult<()> {
		unimplemented!()
	}
}
