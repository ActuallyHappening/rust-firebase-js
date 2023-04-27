# Firebase JS sys
This crate provides a low-level wrapper around the Firebase JS SDK, the useful
functions and types are `#[wasm_bindgen]`ed into Rust, keeping JS semantics.

This package intends to be a solid foundation for building onto of, with
extensive tests and common known errors mapped into Rust.

Contributions welcome!

## Aim
To provide the very first 'glue' between Rust and JS for firebase.
Any errors emenating from this library should be soley from the JavaScript world,
caused by an incorrect value being passed into a *rust* function that should
be interpretted as a *JS* function.

This library is definitely not practical to use, is is designed to provide a
stable base to build other libraries on top of, notably `firebase-js`.