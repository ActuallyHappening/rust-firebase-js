use firebase_js::js_semantic::app::{initialize_app, FirebaseConfig};
use wasm_bindgen_test::{wasm_bindgen_test as test};

#[test]
fn empty_initialize_app() {
	initialize_app(&FirebaseConfig::new("foo bar wrong id".to_string()));
}