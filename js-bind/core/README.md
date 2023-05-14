# js-bind-core: Core functionality for `js-bind` crate
Don't use this crate directly, use the `js-bind` crate instead.

This create is required so that the `js-bind-proc` can depend on it, as rust does not allow
procedural macros and other stuff to be exported in the same library crate.