use extract_doctests::extract_doctests;

#[extract_doctests(inline_config(bad_attr = ""))]
fn placeholder() {}

fn main() {}
