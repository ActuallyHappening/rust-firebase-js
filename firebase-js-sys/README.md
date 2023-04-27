# Firebase JS SYS
**Raw wrappers around the `firebase` npm package** for `Rust` consumption.
Delicious!

See [the `firebase-js` package](https://github.com/ActuallyHappening/rust-firebase-js/tree/master/firebase-js)
for a high level implementation layer ontop of this crate, which is probably what you want rather
than having to manually deal with each `JsValue`.

## Design
This crate exposes only the VERY raw bindings to the underlying JS!