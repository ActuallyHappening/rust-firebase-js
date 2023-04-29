use syn::ItemFn;
use wasm_bindgen::JsValue;

fn main() {
	struct CustomType;

	struct ReturnType;

	#[firebase_js_sys_proc::nothing]
	fn function_name(argument_label1: String, argument_label2: CustomType) -> ReturnType {}

	yes();
}
