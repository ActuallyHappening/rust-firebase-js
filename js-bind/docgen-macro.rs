#[::wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
	#[doc = " Documentation of func"]
	#[doc = " "]
	#[doc = " ```rust"]
	#[doc = " // JS BIND-TEST example_test_name"]
	#[doc = " assert_eq!(\"Yes this test executed well!\", \"\")"]
	#[doc = " ```"]
	#[wasm_bindgen(js_name = "alert")]
	fn alert3(s: &str);
}
