use syn::*;

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeContext {
// 	function_name: String,
// 	js_function_name: String,
// 	js_module_name: String,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeBlock {
// 	lines: Vec<String>,
// 	parameters: CodeParams,
// 	context: CodeContext,
// }

// impl CodeBlock {
// 	pub fn new(lines: Vec<String>, context: CodeContext) -> CodeBlock {
// 		Self {
// 			// TODO: not clone for performance?
// 			lines: lines.clone(),
// 			parameters: CodeParams::from_strings(lines),
// 			context,
// 		}
// 	}
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct CodeParams {
// 	lang_identifier: Option<String>,
// 	test_option: Option<String>,
// 	other: Option<Vec<String>>
// }

// impl CodeParams {
// 	/// Transforms a string vector into a CodeParams struct
// 	/// 
// 	pub fn from_strings(start_line: String) -> CodeParams {
// 		let items: Vec<&str> = start_line.trim().to_owned().split(",").collect();
// 		Self {
// 			// TODO: Not clone for performance?
// 			lang_identifier: items.first().map(|f| f.to_owned()),
// 			test_option: items.get(1).cloned(),
// 			other: items.get(2..).map(|v| v.to_vec()),
// 		}
// 	}
// }

fn _handle_doc(doc: Vec<String>) {
	// eprintln!("Doc: {:?}", doc);

	// Extract ```rs ``` code blocks
	let mut code_blocks: Vec<Vec<String>> = Vec::new();
	let mut in_code_block = false;
	let mut code_block: Vec<String> = Vec::new();
	for line in doc {
		let line = line.trim();
		if line.starts_with("```") {
			if in_code_block {
				// Exiting code block
				code_block.push(line.to_owned());
				code_blocks.push(code_block);
				code_block = Vec::new();
				in_code_block = false;
			} else {
				// Entering code block
				code_block = Vec::new(); // Repetition of line 4 above
				code_block.push(line.to_owned());
				in_code_block = true;
			}
		} else if in_code_block {
			// In code block, not boundary
			code_block.push(line.to_owned());
		}
	}

	eprintln!("Code blocks: {:#?}", code_blocks);
}

fn handle_doc_comments(func: &ItemFn) {
	// TODO: Not clone? maybe expensive?
	let mut doc_comments = Vec::new();
	(func.attrs.clone()).into_iter().for_each(|attr| {
		eprintln!("Attr: {:#?}", attr);
		if let Meta::NameValue(meta_name_value) = attr.meta {
			if meta_name_value.path.is_ident("doc") {
				match meta_name_value.value {
					Expr::Lit(ExprLit { lit: Lit::Str(doc), .. }) => {
						doc_comments.push(doc.value());
					},
					_ => {}
				}
			}
		}
	});
	_handle_doc(doc_comments);
}

