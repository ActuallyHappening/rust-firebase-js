

/// Structure representing Firebase error
/// 
/// # Example
/// The following example 
/// ```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct FirebaseError {
	/// The error code for this error.
	/// ```ts
	/// readonly code: string
	/// ```
	code: String,
	/// Custom data for this error.
	/// ```ts
	/// customData?: Record<string, unknown> | undefined;
	/// ```
	#[serde(rename = "custom_data")]
	customData: Option<HashMap<String, JsValue>>,
}