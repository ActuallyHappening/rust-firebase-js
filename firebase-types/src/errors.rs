use extract_doctests::extract_doctests;
use js_sys::{Object, Reflect};
use smart_default::SmartDefault;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};

/// Represents the underlying generic JS firebase error.
#[derive(Debug, Clone, SmartDefault, PartialEq)]
pub struct JsFirebaseError {
	/// The error code for this error.
	/// ```ts
	/// readonly code: string
	/// ```
	#[default("default/default")]
	pub code: String,

	/// The error message.
	#[default("DEFAULT ERROR MESSAGE")]
	pub message: String,

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
		let err = js
			.dyn_ref::<js_sys::Error>()
			.ok_or(JsValue::from("Cannot cast obj to JS error type."))?;
		let code = Reflect::get(&js, &JsValue::from_str("code"))?
			.as_string()
			.ok_or(JsValue::from("Cannot get .code from obj as string"))?;
		let message = err.message().into();
		let custom_data = match Reflect::get(&js, &JsValue::from("custom_data")) {
			Ok(_val) => Some(
				// Object::entries(&val.dyn_into::<js_sys::Object>()?)
				// 	.iter()
				// 	.map(|obj| {
				// 		obj

				// 	}),
				// add debug statement:
				{
					let mut map = HashMap::new();
					map.insert("TODO".to_string(), "implement actual custom data parsing".to_string());
					map
				},
			),
			Err(_) => None,
		};
		// .map(|obj| {
		// 	obj
		// 		.entries()
		// 		.iter()
		// 		.map(|(k, v)| {
		// 			(
		// 				k.as_string()
		// 					.ok_or(JsValue::from("Cannot get key as string"))?,
		// 				v.as_string()
		// 					.ok_or(JsValue::from("Cannot get value as string"))?,
		// 			)
		// 		})
		// 		.collect::<HashMap<String, String>>()
		// });
		Ok(Self {
			code,
			message,
			custom_data,
			js,
		})
	}
}

/// An extra layer of type safety on top of [JsFirebaseError].
#[extract_doctests]
#[derive(Debug, Clone, SmartDefault, PartialEq)]
pub enum FirebaseAppError {
	/// Error type, when trying to initialize an app but didn't pass any options.
	/// If using js, this is equivalent to a [JsFirebaseError] with code = "app/no-options".
	///
	/// ## JS Example:
	/// ```rust,no_run
	/// // extract-doctests err_app_no_options
	/// use firebase_js_sys::app::initialize_app;
	/// use firebase_types::{FirebaseAppError, JsFirebaseError};
	/// use wasm_bindgen::JsValue;
	///
	/// let err: JsValue = initialize_app(JsValue::UNDEFINED, JsValue::UNDEFINED).expect_err("didn't err?");
	/// let err: JsFirebaseError = err.try_into().expect("Failed to read .code and .custom_data");
	/// let err: FirebaseAppError = err.into();
	/// match err {
	/// 		FirebaseAppError::AppNoOptions(err) => {
	/// 				// Do something with err
	/// 		},
	/// 		_ => panic!("Expected AppNoOptions error, got {:?}", err),
	/// }
	/// ```
	AppNoOptions(JsFirebaseError),

	#[default]
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
