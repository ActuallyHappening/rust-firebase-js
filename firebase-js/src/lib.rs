//! `firebase-js` wraps the Firebase JavaScript SDK for use in Rust and WebAssembly.
//!
//! ## WIP:
//! Currently, no errors are handled :(

use app::App;
use firebase_js_sys::database::DatabaseSnapshot;
use js_sys::Error;
use thiserror::Error as DeriveError;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure};

type TClosure<Args> = Closure<dyn FnMut(Args)>;
pub type FResult<T> = std::result::Result<T, FirebaseError>;

pub mod app;
pub mod db;

#[derive(Debug)]
/// Represents the lifetime of closures passed to the Firebase JS SDK.
///
/// TODO: Make generic over any type
pub struct Global<'this> {
	app: App<'this>,
	closures: Vec<&'this Closure<dyn FnMut(DatabaseSnapshot)>>,
}

impl<'this> Global<'this> {
	pub fn new(app: App<'this>) -> Self {
		Self {
			app,
			closures: Vec::new(),
		}
	}

	pub fn register_closure<T: FromWasmAbi>(closure: impl FnMut(T)) {
		let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut(T)>);
	}

	fn take_closure<T>(closure: Closure<dyn FnMut(T)>) {}
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
