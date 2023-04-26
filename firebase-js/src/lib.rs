//! `firebase-js` wraps the Firebase JavaScript SDK for use in Rust and WebAssembly.
//! 
//! ## WIP:
//! Currently, no errors are handled :(

use thiserror::Error;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsValue};

#[allow(non_camel_case_types)]
type closure<Args> = Closure<dyn FnMut(Args)>;

pub mod js_semantic;
pub mod rusty;

#[derive(Debug, Error)]
pub enum FirebaseError {
	#[error("No errors implemented yet!")]
	UnimplementedErrorHandling,

	#[error("Unhandled raw `JsValue`, this is the library maintainer's fault :)")]
	RawJsValueError(JsValue),
}