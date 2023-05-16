use extract_doctest::extract_doctest;

#[extract_doctest(inline_config(template = ""))]
type Test = ();

fn main() {}