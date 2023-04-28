//! `firebase_js_sys` is a wrapper around the firebase javascript sdk, allowing you to use it in rust.
//! 
//! See [this package's README](https://github.com/ActuallyHappening/rust-firebase-js/blob/master/firebase-js-sys/README.md) for more information,
//! and [the `firebase-js` package](https://github.com/ActuallyHappening/rust-firebase-js/tree/master/firebase-js) for a high level implementation
//! layer ontop of this crate.
//! 
//! Don't use this crate unless you are prepared to implement a lot more conversion logic, as most
//! of the functions return `JsValue`s instead of a more useful types.
//! See crate `firebase-js` for a more ergonomic interface.
//! 
//! Uses bindings hand-written using `#[wasm_bindgen]` and `rollup`.
//! 
//! ## Examples:
//! Usage in general:
//! ```rs
//! use firebase_js_sys::app;
//! 
//! // Will give runtime console error
//! app::initialize_app(&JsValue::UNDEFINED);
//! ```
//! 
//! Example main.rs for using `trunk` to build + run in browser:
//! ```rs
//! use log::info;
//! use wasm_bindgen::JsValue;
//! 
//! fn main() {
//! 	_ = console_log::init_with_level(log::Level::Debug);
//! 	
//! 	console_error_panic_hook::set_once();
//! 
//! 	info!("main.rs is running!");
//! 	
//! 	// Will not work, but only gives console run time error
//! 	let app = firebase_js_sys::ModuleApp::initialize_app(&JsValue::UNDEFINED);
//! 	// println!("returned: {:?}", app);
//! }
//! ```
//! ## Features:
#![doc = document_features::document_features!()]

// compile error is niether feature is enabled
#[cfg(not(any(feature = "web-not-node", feature = "node-not-web")))]
compile_error!("You must enable either the `web-not-node` or `node-not-web` feature to use this crate");
// compile error if both features are enabled
// #[cfg(all(feature = "web-not-node", feature = "node-not-web"))]
// compile_error!("You must enable either the `web-not-node` or `node-not-web` feature to use this crate, not both");


/// Tests doctests in README when running `cargo test`, see:
/// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub mod app;
pub use app::*;