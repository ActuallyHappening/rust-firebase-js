use std::{collections::HashMap};
use extract_doctests::extract_doctests;
use smart_default::SmartDefault;
use wasm_bindgen::{JsValue};
use js_sys::Reflect;

/// Represents the underlying generic JS firebase error.
#[derive(Debug, Clone, SmartDefault, PartialEq)]
pub struct JsFirebaseError {
	/// The error code for this error.
	/// ```ts
	/// readonly code: string
	/// ```
	#[default("default/default")]
	pub code: String,

	/// Custom data for this error.
	/// ```ts
	/// customData?: Record<string, unknown> | undefined;
	/// ```
	pub custom_data: Option<HashMap<String, String>>,

	// #[serde(skip)]
	js: JsValue,
}

impl JsFirebaseError {
	pub fn new_from_code(code: &str) -> Self {
		Self {
			code: code.to_string(),
			..Default::default()
		}
	}
}

// impl AsRef<JsValue> for JsFirebaseError {
// 	fn as_ref(&self) -> &JsValue {
// 		&self.js
// 	}
// }

impl TryFrom<JsValue> for JsFirebaseError {
	type Error = JsValue;

	fn try_from(js: JsValue) -> Result<Self, Self::Error> {
		Ok(Self {
			code: Reflect::get(&js, &JsValue::from_str("code"))?.as_string().unwrap(),
			js,
			..Default::default()
		})
	}
}

#[extract_doctests]
pub enum FirebaseAppError {
	/// Error type, when trying to initialize an app but didn't pass any options.
	/// If using js, this is equivalent to a [JsFirebaseError] with code = "app/no-options".
	/// 
	/// ## JS Example:
	/// ```rust,no_run
	/// // extract-doctests err_app_no_options
	/// use firebase_js_sys::app::initialize_app;
	/// 
	/// let err: FirebaseAppError = initialize_app(JsValue::UNDEFINED, JsValue::UNDEFINED).expect_err("didn't err?").into();
	/// match err {
	/// 		FirebaseAppError::AppNoOptions(err) => {
	/// 				// Do something with err
	/// 		},
	/// 		_ => panic!("Expected AppNoOptions error, got {:?}", err),
	/// }
	/// ```
	AppNoOptions(JsFirebaseError),

	Unknown(JsFirebaseError),
}

impl From<JsFirebaseError> for FirebaseAppError {
	fn from(err: JsFirebaseError) -> Self {
		if err.code == "app/no-options" {
			Self::AppNoOptions(err)
		} else {
			Self::Unknown(err)
		}
	}
}