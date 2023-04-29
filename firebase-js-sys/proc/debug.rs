#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use syn::ItemFn;
use wasm_bindgen::JsValue;
fn main() {
    struct CustomType;
    struct ReturnType;
    use wasm_bindgen::JsValue;
    extern "C" {
        #[allow(non_camel_case_types)]
        #[::wasm_bindgen(js_name = "app")]
        type _app;
        /// Takes a config object and returns a firebase app instance
        ///
        /// Equivalent to:
        /// ```js
        /// import { initializeApp } from 'firebase/app';
        ///
        /// // Get your own config from somewhere, typically copy-paste from firebase console
        /// const config = {
        /// 	apiKey: "...",
        /// 	projectId: "...",
        /// 	...
        /// }
        ///
        /// initializeApp(config);
        /// ```
        ///
        #[::wasm_bindgen(catch, static_method_of = _app, js_name = "initializeApp")]
        pub fn initialize_app(
            config: &JsValue,
            name: Option<String>,
        ) -> Result<JsValue, JsValue>;
    }
}
