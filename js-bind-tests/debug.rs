#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use js_bind::js_bind;
use wasm_bindgen::prelude::wasm_bindgen;
use js_bind::Config;
extern "C" {
    /// Documentation!
    pub fn log(msg: String);
}
fn main() {}
