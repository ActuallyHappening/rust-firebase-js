#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasm_bindgen::JsValue;
use firebase_js_sys_proc::js_bind;
fn main() {
    struct _CustomType;
    struct _ReturnType;
    ();
    extern "C" {}
    fn function_name(
        argument_label1: String,
        argument_label2: u64,
    ) -> Result<i32, JsValue> {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    format_args!(
                        "Calling function: {0}::{1}(<parameters pass not implemented yet>)",
                        "_mod", "function_name"
                    ),
                    lvl,
                    &(
                        "firebase_js_sys_proc",
                        "firebase_js_sys_proc",
                        "firebase-js-sys/proc/src/main.rs",
                        9u32,
                    ),
                    ::log::__private_api::Option::None,
                );
            }
        };
        _mod::function_name(argument_label1, argument_label2)
    }
    function_name("ff".to_string(), 69u64);
}
