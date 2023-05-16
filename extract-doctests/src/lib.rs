#![doc = include_str!("../README.md")]

pub use extract_doctests_proc::extract_doctests;

#[test]
fn ui_tests() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui_*.rs");
}