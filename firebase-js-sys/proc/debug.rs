#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    struct _CustomType;
    struct _ReturnType;
    use wasm_bindgen::prelude::wasm_bindgen;
    #[allow(nonstandard_style)]
    #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
    ///
    fn function_name(
        argument_label1: String,
        argument_label2: _CustomType,
    ) -> _ReturnType {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __wbg_functionname_a535acec05228e54(
            argument_label1: <String as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            argument_label2: <_CustomType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <_ReturnType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(argument_label1);
            drop(argument_label2);
            {
                ::std::rt::begin_panic(
                    "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
                )
            };
        }
        unsafe {
            let _ret = {
                let argument_label1 = <String as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    argument_label1,
                );
                let argument_label2 = <_CustomType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                    argument_label2,
                );
                __wbg_functionname_a535acec05228e54(argument_label1, argument_label2)
            };
            <_ReturnType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
