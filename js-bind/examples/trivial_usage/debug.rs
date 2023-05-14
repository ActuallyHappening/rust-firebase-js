#![feature(prelude_import)]
#![allow(dead_code, improper_ctypes)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use js_bind_proc::js_bind;
#[allow(nonstandard_style)]
#[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
/** Documentation of func

 ```rust
 assert_eq!("Yes this test executed well!", "")
 ```*/
fn alert3(s: &str) {
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    unsafe fn __wbg_alert_9e6ac9cf24f8dfef(
        s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    ) -> () {
        drop(s);
        {
            ::std::rt::begin_panic(
                "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
            )
        };
    }
    unsafe {
        let _ret = {
            let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
            __wbg_alert_9e6ac9cf24f8dfef(s)
        };
        ()
    }
}
fn main() {
    alert3("Hello, world!");
}
documentation: [" Documentation of func", " ", " ```rust", " assert_eq!(\"Yes this test executed well!\", \"\")", " ```"]
testable_code_blocks: []
#![feature(prelude_import)]
#![allow(dead_code, improper_ctypes)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use js_bind_proc::js_bind;
#[allow(nonstandard_style)]
#[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
/** Documentation of func

 ```rust
 assert_eq!("Yes this test executed well!", "")
 ```*/
fn alert3(s: &str) {
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    unsafe fn __wbg_alert_9e6ac9cf24f8dfef(
        s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    ) -> () {
        drop(s);
        {
            ::std::rt::begin_panic(
                "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
            )
        };
    }
    unsafe {
        let _ret = {
            let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
            __wbg_alert_9e6ac9cf24f8dfef(s)
        };
        ()
    }
}
fn main() {
    alert3("Hello, world!");
}
