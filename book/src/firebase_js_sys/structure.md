# Structure
This crate exposes two main modules, `js` and `helpers`.

## `js` module
This module contains the actual function bindings, returning `JsValue`s and always
having errors wrapped in `Result`s.
Often the functions here are quite impractical, requiring knowledge of `wasm-bindgen`,
`Closure`s and `JsValue`s.

## Why another wrapping module is needed
Even though this package ends in `-sys`, which by convention exposes only the 'raw'
bindings without adding another level of abstraction, these are the reasons I believe
another layer is needed:

### Abitrary error types
In the 'raw' bindings (`js` module), errors are returned as `JsValue`s, which is
not very 'stable' to work with esspecially if there are only a select few types of
errors which occue 99% of the time.
Adding a statically typed layer of error handling falls in the domain of this crate
because exposing `JsValue` in the return type of a function conceptually requires
an understanding (however basic) of JavaScript!

### Summary
Using this library should establish a safe interop between `JS` and `Rust`, rather
than leaving that to users of this crate.
This is the reasoning behind the `helpers` module, which is *not* a high level wrapper,
but rather a low level interpretation of the JS SDK.

## `helpers` module
This module contains wrappers for errors returned