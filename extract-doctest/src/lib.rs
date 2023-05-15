// #![doc = include_str!("../README.md")]

pub use extract_doctest_proc::extract_doctest;

#[test]
fn ui_tests() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/*.rs");
}