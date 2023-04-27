# Package relationships
`firebase-js` depends on:
- `firebase-js-wrappers` for type wrappers
- `firebase-js-sys` for the actual bindings IF feature flag `sys` is enabled

`firebase-js-sys` depends on:
- `firebase-js-wrappers` for type wrappers
- `wasm-bindgen`, `js-sys` e.t.c. for the actual bindings
- Bunch of hand-written `js` code and the `npm` package `firebase`

`firebase-js-wrappers` depends on: Nothing important.
