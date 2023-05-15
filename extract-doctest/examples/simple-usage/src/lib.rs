#![allow(dead_code)]
#![allow(unused_imports)]

use extract_doctest::extract_doctest;

/// Documentation
/// ```rust
/// // extract-doctest name_of_func
/// assert_eq!(1, 23);
/// ```
#[extract_doctest]
pub fn nothing() {

}

#[cfg(test)]
mod test {
	use super::*;

	fn test_func_exists() {
		name_of_func();
	}
}