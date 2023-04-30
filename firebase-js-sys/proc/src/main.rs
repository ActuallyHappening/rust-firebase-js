use wasm_bindgen::JsValue;

fn main() {
	struct _CustomType;

	struct _ReturnType;

	#[firebase_js_sys_proc::js_bind("mod")]
	/// Documentation of macro usage func
	fn function_name(argument_label1: String, argument_label2: u64) -> Result<i32, JsValue> {
		let code = "this is literaly code";
	}

	// yes();
	function_name("ff".to_string(), 69u64);
}
