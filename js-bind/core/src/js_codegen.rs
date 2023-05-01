use crate::config::{Mode, Build};



/// Represents a file that is produced by JS codegen
pub struct JsOutputFile {
}

impl JsOutputFile {
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