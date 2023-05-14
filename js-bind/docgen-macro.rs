#[::wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
	#[doc = " Documentation of func"]
	#[doc = " "]
	#[doc = " ```rust"]
	#[doc = " // JSBIND-TEST example_test_name"]
	#[doc = " assert_eq!(\"Yes this test executed well!\", \"\")"]
	#[doc = " ```"]
	#[wasm_bindgen(js_name = "alert")]
	fn alert3(s: &str);
}
#[::wasm_bindgen_test::wasm_bindgen_test]
fn example_test_name() {
	# [cfg (feature = # web_feature_name)]
	::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
	assert_eq!("Yes this test executed well!", "")
}
