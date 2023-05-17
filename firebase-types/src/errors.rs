use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a generic JS firebase error.
/// 
/// # Example
/// The following example 
/// ```rust
/// 
/// ```
#[cfg(feature = "js")]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct JsFirebaseError {
	/// The error code for this error.
	/// ```ts
	/// readonly code: string
	/// ```
	code: String,
	/// Custom data for this error.
	/// ```ts
	/// customData?: Record<string, unknown> | undefined;
	/// ```
	custom_data: Option<HashMap<String, String>>,
}