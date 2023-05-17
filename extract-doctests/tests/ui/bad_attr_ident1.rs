use extract_doctests::extract_doctests;

#[extract_doctests(bad_attr(template = ""))]
fn placeholder() {}

fn main() {}
