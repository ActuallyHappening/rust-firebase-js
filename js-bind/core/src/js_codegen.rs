use crate::config::{Mode, Build};



pub struct JsCodeBlock {
	lines: Vec<String>
}

impl JsCodeBlock {
	pub fn new(name: String, mode: Mode, build_options: Build) {
		match mode.item_type.as_str() {
			"function" => {

			}
			_ => {
				panic!("Unsupported js_item_type: {:?}", mode.item_type)
			}
		}
	}
}