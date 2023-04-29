#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasm_bindgen::JsValue;
fn main() {
    struct _CustomType;
    struct _ReturnType;
    use wasm_bindgen::prelude::wasm_bindgen;
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    ///
    #[repr(transparent)]
    struct _app {
        obj: wasm_bindgen::JsValue,
    }
    #[automatically_derived]
    const _: () = {
        use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
        use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
        use wasm_bindgen::convert::{RefFromWasmAbi, LongRefFromWasmAbi};
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsValue, JsCast, JsObject};
        use wasm_bindgen::__rt::core;
        impl WasmDescribe for _app {
            fn describe() {
                JsValue::describe()
            }
        }
        impl IntoWasmAbi for _app {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for _app {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a _app {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for _app {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                _app {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for _app {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a _app {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for _app {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<_app>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(_app {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for _app {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = _app;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                _app { obj: tmp.into() }
            }
        }
        impl From<JsValue> for _app {
            #[inline]
            fn from(obj: JsValue) -> _app {
                _app { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for _app {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<_app> for _app {
            #[inline]
            fn as_ref(&self) -> &_app {
                self
            }
        }
        impl From<_app> for JsValue {
            #[inline]
            fn from(obj: _app) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for _app {
            fn instanceof(val: &JsValue) -> bool {
                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                unsafe fn __wbg_instanceof__app_91767560bbbc1672(_: u32) -> u32 {
                    {
                        ::std::rt::begin_panic(
                            "cannot check instanceof on non-wasm targets",
                        )
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof__app_91767560bbbc1672(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                _app { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const _app) }
            }
        }
        impl JsObject for _app {}
    };
    #[automatically_derived]
    impl core::ops::Deref for _app {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[automatically_derived]
    impl _app {
        #[allow(nonstandard_style)]
        #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
        ///
        fn function_name(
            argument_label1: String,
            argument_label2: u64,
        ) -> Result<i32, JsValue> {
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __wbg_initializeApp_f7aa4b07af0a6cc9(
                argument_label1: <String as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                argument_label2: <u64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    let argument_label2 = <u64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        argument_label2,
                    );
                    __wbg_initializeApp_f7aa4b07af0a6cc9(
                        argument_label1,
                        argument_label2,
                    )
                };
                wasm_bindgen::__rt::take_last_exception()?;
                Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
            }
        }
    }
}
