/// TS type:
/// /** The error code for this error. */
/// readonly code: string
///  /** Custom data for this error. */
/// customData?: Record<string, unknown> | undefined;
pub struct FirebaseError {
	code: String,
	customData: Option<HashMap<String, JsValue>>,
}