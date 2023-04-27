# Structure
This crate exposes two main modules, `js` and `helpers`.

## `js` module
This module contains the actual function bindings, returning `JsValue`s and always
having errors wrapped in `Result`s.
Often the functions here are quite impractical, requiring knowledge of `wasm-bindgen`,
`Closure`s and `JsValue`s.