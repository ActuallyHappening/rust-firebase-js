#![allow(dead_code)]
#![allow(unused_imports)]

use extract_doctest::extract_doctest;

/// Documentation
/// ```rust,should_panic
/// // extract-doctest name_of_func
/// 
/// // This statement is only executed by rustdoc because
/// // of the template that is used, see Cargo.toml
/// assert_eq!(1, 23);
/// 
/// ```
#[extract_doctest]
pub fn nothing() {

}

pub fn test() {
	name_of_func();
}