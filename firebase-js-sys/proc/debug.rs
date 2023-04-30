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
    struct _mod {
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
        impl WasmDescribe for _mod {
            fn describe() {
                JsValue::describe()
            }
        }
        impl IntoWasmAbi for _mod {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for _mod {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a _mod {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for _mod {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                _mod {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for _mod {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a _mod {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for _mod {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<_mod>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(_mod {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for _mod {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = _mod;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                _mod { obj: tmp.into() }
            }
        }
        impl From<JsValue> for _mod {
            #[inline]
            fn from(obj: JsValue) -> _mod {
                _mod { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for _mod {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<_mod> for _mod {
            #[inline]
            fn as_ref(&self) -> &_mod {
                self
            }
        }
        impl From<_mod> for JsValue {
            #[inline]
            fn from(obj: _mod) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for _mod {
            fn instanceof(val: &JsValue) -> bool {
                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                unsafe fn __wbg_instanceof__mod_ac2c4546e8f42af7(_: u32) -> u32 {
                    {
                        ::std::rt::begin_panic(
                            "cannot check instanceof on non-wasm targets",
                        )
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof__mod_ac2c4546e8f42af7(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                _mod { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const _mod) }
            }
        }
        impl JsObject for _mod {}
    };
    #[automatically_derived]
    impl core::ops::Deref for _mod {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[automatically_derived]
    impl _mod {
        #[allow(nonstandard_style)]
        #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
        ///
        fn function_name(
            argument_label1: String,
            argument_label2: u64,
        ) -> Result<i32, JsValue> {
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __wbg_initializeApp_3104b3635720be45(
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
                    __wbg_initializeApp_3104b3635720be45(
                        argument_label1,
                        argument_label2,
                    )
                };
                wasm_bindgen::__rt::take_last_exception()?;
                Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
            }
        }
    }
    fn function_name(
        argument_label1: String,
        argument_label2: u64,
    ) -> Result<i32, JsValue> {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    format_args!("Calling function: {0}::{1}", "_mod", "function_name"),
                    lvl,
                    &(
                        "firebase_js_sys_proc",
                        "firebase_js_sys_proc",
                        "firebase-js-sys/proc/src/main.rs",
                        8u32,
                    ),
                    ::log::__private_api::Option::None,
                );
            }
        };
        _mod::function_name(argument_label1, argument_label2)
    }
    function_name("ff".to_string(), 69u64);
}
