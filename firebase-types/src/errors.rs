use extract_doctests::extract_doctests;
use js_sys::Reflect;
use smart_default::SmartDefault;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

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
	#[default(js_sys::Error::new("DEFAULT JS ERROR"))]
	js: js_sys::Error,
}

impl JsFirebaseError {
	pub fn new_from_code(code: &str) -> Self {
		Self {
			code: code.to_string(),
			..Default::default()
		}
	}
}

impl TryFrom<js_sys::Error> for JsFirebaseError {
	type Error = JsValue;

	fn try_from(js: js_sys::Error) -> Result<Self, Self::Error> {
		// let err = js
		// 	.dyn_ref::<js_sys::Error>()
		// 	.ok_or(JsValue::from("Cannot cast obj to JS error type."))?;

		let code = Reflect::get(&js, &JsValue::from_str("code"))?
			.as_string()
			.ok_or(JsValue::from("Cannot get .code from obj as string"))?;
		let message = js.message().into();
		let custom_data = match Reflect::get(&js, &JsValue::from("custom_data")) {
			Ok(_val) => Some(
				// add debug statement:
				{
					let mut map = HashMap::new();
					map.insert(
						"TODO".to_string(),
						"implement actual custom data parsing".to_string(),
					);
					map
				},
			),
			Err(_) => None,
		};
		Ok(Self {
			code,
			message,
			custom_data,
			js,
		})
	}
}