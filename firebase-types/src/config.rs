use serde::{Serialize, Deserialize};
#[cfg(feature = "expose-jsvalue")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "expose-jsvalue")]
use serde_wasm_bindgen::*;

/// ## Represents a Firebase config object.
/// This object **is serializable to JS** as you expect.
/// I suggest constructing this object using the [FirebaseConfigConstructor] type,
/// as it is easier to copy-paste from JS.
/// 
/// ### Constructors
/// - [`FirebaseConfig::new`]
/// - [`FirebaseConfigConstructor::into_config().expect("projectId to be provided")`]
/// 
/// ## Example:
/// Constructing with just a project id:
/// ```rust
/// use firebase_types::config::FirebaseConfig;
/// 
/// let config: FirebaseConfig = FirebaseConfig::new("some-project-id".to_owned());
/// 
/// assert_eq!(config.project_id, "some-project-id");
/// ```
/// Constructing with a project id and other fields:
/// ```rust
/// use firebase_types::config::FirebaseConfig;
/// 
/// let config: FirebaseConfig = FirebaseConfig {
/// 	project_id: "some-project-id".to_owned(),
/// 	database_url: Some("https://some-project-id.firebaseio.com".to_owned()),
/// 	app_id: Some("some-app-id".to_owned()),
/// 	measurement_id: Some("some-measurement-id".to_owned()),
/// 	..Default::default()
/// };
/// 
/// assert_eq!(config.project_id, "some-project-id");
/// assert_eq!(config.database_url, Some("https://some-project-id.firebaseio.com".to_owned()));
/// assert_eq!(config.app_id, Some("some-app-id".to_owned()));
/// assert_eq!(config.measurement_id, Some("some-measurement-id".to_owned()));
/// ```
/// Serializing into a [JsValue] (requires feature `expose-jsvalue`)
/// ```rust,no_run
/// use firebase_types::config::FirebaseConfig;
/// use wasm_bindgen::JsValue;
/// 
/// let config = FirebaseConfig::new("some-project-id");
/// 
/// let raw_js_value: JsValue = config.try_into().expect("serde_wasm_bindgen to work");
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FirebaseConfig {
	#[serde(rename = "projectId")]
	pub project_id: String,

	// #[new(default)]
	#[serde(rename = "apiKey")]
	pub api_key: Option<String>,

	// #[new(default)]
	#[serde(rename = "authDomain")]
	pub auth_domain: Option<String>,

	// #[new(default)]
	#[serde(rename = "storageBucket")]
	pub storage_bucket: Option<String>,

	// #[new(default)]
	#[serde(rename = "messagingSenderId")]
	pub messaging_sender_id: Option<String>,

	// #[new(default)]
	#[serde(rename = "appId")]
	pub app_id: Option<String>,

	// #[new(default)]
	#[serde(rename = "measurementId")]
	pub measurement_id: Option<String>,

	// #[new(default)]
	#[serde(rename = "databaseURL")]
	pub database_url: Option<String>,
}

impl FirebaseConfig {
	pub fn new(project_id: impl ToString) -> Self {
		Self {
			project_id: project_id.to_string(),
			..Default::default()
		}
	}
}

impl TryFrom<FirebaseConfig> for JsValue {
	type Error = JsValue;

	fn try_from(value: FirebaseConfig) -> Result<Self, Self::Error> {
		serde_wasm_bindgen::to_value(&value).map_err(|e| e.into())
	}
}

/// ## A helper type for constructing a [FirebaseConfig].
/// This is useful when copy-pasting from JS objects, as minimal meta-change is requried.
/// 
/// If you tried to construct a [FirebaseConfig] directly, you would need to make many changes!
/// See example below for what I mean.
/// 
/// ## Example:
/// ```js
/// // Imagine you had this firebase config object:
/// const config = {
///		projectId: "some-project-id" 
///		databaseURL: "https://some-project-id.firebaseio.com",
/// }
/// ```
/// ```rust
/// use firebase_types::config::{FirebaseConfigConstructor, FirebaseConfig};
/// 
/// // If you copy-paste this from js, add a `..Default::default()` to the end.
/// let helper_config = FirebaseConfigConstructor {
///		projectId: "some-project-id",
/// 	databaseURL: "https://some-project-id.firebaseio.com",
/// 
/// 	// Add this line if you copy-paste from JS 
/// 
/// 	..Default::default() // <-- The magic line
/// };
/// 
/// // Convert into project [FirebaseConfig] object.
/// // This fill fail if 'projectId' is not provided
/// let config: FirebaseConfig = helper_config.into_config().expect("projectId to be provided");
/// 
/// assert_eq!(config.project_id, "some-project-id");
/// ```
/// 
/// ## Why
/// If you tried to construct a [FirebaseConfig] directly and you had many extra fields,
/// you would end up writing a lot of boilerplate code.
/// Also, the names of the fields in [FirebaseConfigConstructor] are the same as the JS object,
/// so no renaming is requried.
/// ```js
/// // Imagine you had this firebase config object:
/// const config = {
/// 	projectId: "some-project-id"
/// 	databaseURL: "https://some-project-id.firebaseio.com",
/// 	appId: "some-app-id",
/// 	measurementId: "some-measurement-id",
/// }
/// ```
/// Using [FirebaseConfig] (verbose):
/// ```rust
/// use firebase_types::config::FirebaseConfig;
/// 
/// let config: FirebaseConfig = FirebaseConfig {
/// 	// Note how the field names are different, you would have to change this when copy-pasting from JS
/// 	project_id: "some-project-id".to_owned(),
/// 	database_url: Some("https://some-project-id.firebaseio.com".to_owned()),
/// 	app_id: Some("some-app-id".to_owned()),
/// 	measurement_id: Some("some-measurement-id".to_owned()),
/// 	..Default::default()
/// };
/// 
/// assert_eq!(config.project_id, "some-project-id");
/// assert_eq!(config.database_url, Some("https://some-project-id.firebaseio.com".to_owned()));
/// assert_eq!(config.app_id, Some("some-app-id".to_owned()));
/// assert_eq!(config.measurement_id, Some("some-measurement-id".to_owned()));
/// ```
/// Using [FirebaseConfigConstructor] (less verbose):
/// ```rust
/// use firebase_types::config::{FirebaseConfigConstructor, FirebaseConfig};
/// 
/// let config: FirebaseConfig = FirebaseConfigConstructor {
///		// Not how the field names are the same as JS
/// 	projectId: "some-project-id",
/// 	databaseURL: "https://some-project-id.firebaseio.com",
/// 	appId: "some-app-id",
/// 	measurementId: "some-measurement-id",
/// 	..Default::default()
/// }.into_config().expect("projectId to be provided");
/// ```
#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FirebaseConfigConstructor<'a> {
	pub projectId: &'a str,
	pub apiKey: &'a str,
	pub authDomain: &'a str,
	pub storageBucket: &'a str,
	pub messagingSenderId: &'a str,
	pub appId: &'a str,
	pub measurementId: &'a str,
	pub databaseURL: &'a str,
}

impl<'a> FirebaseConfigConstructor<'a> {
	/// ## Convert a [FirebaseConfigConstructor] into a [FirebaseConfig].
	/// 
	/// This type is useful when copy-pasting from a JS config object.
	/// 
	/// ## Returns [None]
	/// Returns [None] if [FirebaseConfigConstructor.projectId] is empty, i.e. `""`.
	/// This field is required for the [FirebaseConfig] type.
	/// 
	/// ## Example
	/// ```
	/// use firebase_types::config::{FirebaseConfigConstructor, FirebaseConfig};
	/// 
	/// let helper_config = FirebaseConfigConstructor {
	/// 	projectId: "my-project-id",
	/// 	..Default::default()
	/// }.into_config().expect("projectId to be provided");
	/// 
	/// let config = FirebaseConfig::new("my-project-id".to_owned());
	/// 
	/// assert_eq!(helper_config, config);
	/// ```
	pub fn into_config(&self) -> Option<FirebaseConfig> {
		if self.projectId == "" {
			// panic!("projectId is required when converting into [FirebaseConfig]; self: {:?}", self);
			return None
		}
		fn if_empty_none(s: &str) -> Option<String> {
			if s == "" {
				None
			} else {
				Some(s.to_owned())
			}
		}
		Some(FirebaseConfig {
			project_id: self.projectId.to_owned(),
			api_key: if_empty_none(self.apiKey),
			auth_domain: if_empty_none(self.authDomain),
			storage_bucket: if_empty_none(self.storageBucket),
			messaging_sender_id: if_empty_none(self.messagingSenderId),
			app_id: if_empty_none(self.appId),
			measurement_id: if_empty_none(self.measurementId),
			database_url: if_empty_none(self.databaseURL),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn random_string() -> String {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		let n: u8 = rng.gen();
		n.to_string()
	}

	#[test]
	pub fn config_constructor_using_default() {
		let rand_item = random_string();
		let config_constructor = FirebaseConfigConstructor {
			projectId: "my-project-id",
			appId: &rand_item,
			..Default::default()
		};

		assert_eq!(config_constructor.projectId, "my-project-id");
		assert_eq!(config_constructor.apiKey, "");
		assert_eq!(config_constructor.authDomain, "");
		assert_eq!(config_constructor.storageBucket, "");
		assert_eq!(config_constructor.messagingSenderId, "");
		assert_eq!(config_constructor.appId, rand_item);
		assert_eq!(config_constructor.measurementId, "");
		assert_eq!(config_constructor.databaseURL, "");
	}

	#[test]
	pub fn config_constructor_minimal_using_default_into_config() {
		let config_constructor = FirebaseConfigConstructor {
			projectId: "my-project-id",
			..Default::default()
		};

		assert_eq!(config_constructor.projectId, "my-project-id");
		assert_eq!(config_constructor.apiKey, "");
		assert_eq!(config_constructor.authDomain, "");
		assert_eq!(config_constructor.storageBucket, "");
		assert_eq!(config_constructor.messagingSenderId, "");
		assert_eq!(config_constructor.appId, "");
		assert_eq!(config_constructor.measurementId, "");
		assert_eq!(config_constructor.databaseURL, "");

		let config = config_constructor.into_config();

		let expected_config = FirebaseConfig {
			project_id: "my-project-id".to_owned(),
			api_key: None,
			auth_domain: None,
			storage_bucket: None,
			messaging_sender_id: None,
			app_id: None,
			measurement_id: None,
			database_url: None,
		};
		assert_eq!(config, Some(expected_config));
	}

	#[test]
	pub fn config_constructor_minimal_using_default_into_config_fails() {
		let config_constructor = FirebaseConfigConstructor {
			// projectId: "my-project-id",
			..Default::default()
		};

		assert_eq!(config_constructor.projectId, "");
		assert_eq!(config_constructor.apiKey, "");
		assert_eq!(config_constructor.authDomain, "");
		assert_eq!(config_constructor.storageBucket, "");
		assert_eq!(config_constructor.messagingSenderId, "");
		assert_eq!(config_constructor.appId, "");
		assert_eq!(config_constructor.measurementId, "");
		assert_eq!(config_constructor.databaseURL, "");

		let config = config_constructor.into_config();
		assert_eq!(config, None);
	}
}

