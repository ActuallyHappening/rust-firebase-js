use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_test as test};
use wasm_bindgen_test::wasm_bindgen_test_configure as configure;
use firebase_js_sys::app;

/// Macro to 'duplicate' a test, so that it can be run in both nodejs and browser
///
/// Usage:
/// ```ignore
/// #[duplicate_test]
/// fn test_name() {
/// 	// Will inject either configure!(run_in_browser) or nothing
/// 	// test code 
/// }
/// ```
/// Outputs:
/// ```ignore
/// #[test]
/// fn test_name_web() {
/// 	configure!(run_in_browser);
/// 	// test code 
/// }
/// 
/// #[test]
/// fn test_name_node() {
/// 	// test code
/// }
/// ```
macro_rules! duplicate_test {
	($tt: item) => {
		#[test]
		fn $tt() {
				configure!(run_in_browser);
				$($tt)*
		}

		#[test]
		fn $tt() {
			$($tt)*
		}
	}
}
// macro_rules! duplicate_test {
// 	($($tt:tt)*) => {
// 		#[test]
// 		fn $tt() {
// 				configure!(run_in_browser);
// 				$($tt)*
// 		}

// 		// #[test]
// 		// fn $tt() {
// 		// 	$($tt)*
// 		// }
// 	}
// }

#[test]
// #[duplicate_test]
fn manual_initialize_app_empty() {
	wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
	let result = app::initialize_app(JsValue::UNDEFINED);
	
	assert!(result.is_err());
	assert!(false)
	// panic!("Error description: {:?}", result.err().unwrap());
}

duplicate_test!(
	fn test_test() {
		assert_eq(true, true);
	}
);
