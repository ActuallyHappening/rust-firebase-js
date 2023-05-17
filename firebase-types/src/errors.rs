use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents the underlying generic JS firebase error.
#[cfg(feature = "js")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JsFirebaseError {
	/// The error code for this error.
	/// ```ts
	/// readonly code: string
	/// ```
	pub code: String,
	/// Custom data for this error.
	/// ```ts
	/// customData?: Record<string, unknown> | undefined;
	/// ```
	pub custom_data: Option<HashMap<String, String>>,
}

#[cfg(feature = "js")]
impl JsFirebaseError {
	pub fn new_from_code(code: &str) -> Self {
		Self {
			code: code.to_string(),
			custom_data: None,
		}
	}
}

pub enum FirebaseError {
	/// Error type, when trying to initialize an app but didn't pass any options.
	/// If using js, this is equivalent to a [JsFirebaseError] with code = "app/no-options".
	/// 
	/// ## JS Example:
	/// ```rust,no_run
	/// use firebase_js_sys::app::initialize_app;
	/// 
	/// let err = initialize_app(JsValue::UNDEFINED, JsValue::UNDEFINED).expect_err("didn't err?");
	/// 
	/// ```
	AppNoOptions(#[cfg(feature="js")] JsFirebaseError),
}