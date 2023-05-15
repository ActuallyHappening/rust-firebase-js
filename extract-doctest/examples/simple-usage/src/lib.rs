use extract_doctest::extract_doctest;

/// Documentation
/// ```rust
/// // extract-test: name_of_func
/// assert_eq!(1, 1);
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