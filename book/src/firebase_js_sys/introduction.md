# Firebase JS SYS
**Raw wrappers around the `firebase` npm package** for `Rust` consumption.
Delicious!

## WARNING: ⚠️ ENABLE `web-not-node` (default) or `node-not-web`, **not both!** ⚠️ 

See [the `firebase-js` package](https://github.com/ActuallyHappening/rust-firebase-js/tree/master/firebase-js)
for a high level implementation layer ontop of this crate, which is probably what you want rather
than having to manually deal with each `JsValue`.

## Recommended Documentation:
Documentation in Rust-Firebase-JS project book: https://actuallyhappening.github.io/rust-firebase-js/firebase_js_sys/introduction.html

## Design
This crate exposes the lowest level bindings to the underlying JS!
That means you will be dealing with `js_sys::Error`, `js_sys::Object`,
`wasm_bindgen::JsValue`, e.t.c. directly.