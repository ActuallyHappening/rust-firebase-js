//! `firebase-js` wraps the Firebase JavaScript SDK for use in Rust and WebAssembly.
//!
//! ## WIP:
//! Currently, no errors are handled :(

use std::cell::RefCell;

use app::App;
use firebase_js_sys::database::DatabaseSnapshot;
use js_sys::Error;
use thiserror::Error as DeriveError;
use wasm_bindgen::{prelude::Closure, convert::FromWasmAbi};

type TClosure<Args> = Closure<dyn FnMut(Args)>;
pub type FResult<T> = std::result::Result<T, FirebaseError>;

pub mod app;
pub mod db;
pub use firebase_types as types;

#[derive(Debug)]
/// Represents the lifetime of closures passed to the Firebase JS SDK.
///
/// TODO: Make generic over any type
pub struct ClosureGlobal<T> {
	closures: RefCell<Vec<Closure<dyn FnMut(T)>>>,
}

// type T = DatabaseSnapshot;

impl<Params> ClosureGlobal<Params> {
	pub fn new() -> Self {
		Self {
			closures: RefCell::new(Vec::new()),
		}
	}

	pub fn take_closure(&self, closure: Closure<dyn FnMut(Params)>) {
		self.closures.borrow_mut().push(closure);
	}
}

#[derive(Debug, DeriveError)]
pub enum FirebaseError {
	#[error("No errors implemented yet!")]
	UnimplementedErrorHandling,

	#[error("Unhandled raw `JsValue`, this is the library maintainer's fault :)")]
	RawJsError(Error),
}

impl From<Error> for FirebaseError {
	fn from(err: Error) -> Self {
		// TODO: Use `firebase-types` to coerce errors
		Self::RawJsError(err)
	}
}
