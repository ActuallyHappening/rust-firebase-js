use firebase_js_sys_proc::js_bind;
use wasm_bindgen::JsValue;

fn main() {

	#[js_bind("mod_name")]
	/// ```rs
	/// Run this please!
	/// ```
	fn some_func_name(param: String) -> Result<i32, JsValue> {}

	some_func_name("".to_string()).ok();
	_mod_name::some_func_name("".to_string()).ok();
}