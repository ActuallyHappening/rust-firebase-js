pub mod app {
    use js_bind::js_bind;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    /** Takes a config object and returns a firebase app instance

 Equivalent to:
 ```js
 import { initializeApp } from 'firebase/app';

 // Get your own config from somewhere, typically copy-paste from firebase console
 const config = {
 	apiKey: "...",
 	projectId: "...",
 	...
 }

 initializeApp(config);
 ```

 ## Examples
 ```rust
 use firebase_js_sys::app;
 use wasm_bindgen::JsValue;

 let config = JsValue::UNDEFINED;
 let returned = app::initialize_app(config);

 assert!(returned.is_err());
 ```*/
    pub fn initialize_app(config: JsValue) -> Result<JsValue, JsValue> {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_initializeApp_e318a46d1a415a98(
            config: <JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(config);
            {
                ::std::rt::begin_panic(
                    "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
                )
            };
        }
        unsafe {
            let _ret = {
                let config = <JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    config,
                );
                __wbg_initializeApp_e318a46d1a415a98(config)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
