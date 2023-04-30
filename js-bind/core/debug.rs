#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use firebase_js_sys_proc::js_bind;
use wasm_bindgen::JsValue;
fn main() {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    ///
    #[repr(transparent)]
    struct _mod_name {
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
        impl WasmDescribe for _mod_name {
            fn describe() {
                JsValue::describe()
            }
        }
        impl IntoWasmAbi for _mod_name {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                self.obj.into_abi()
            }
        }
        impl OptionIntoWasmAbi for _mod_name {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl<'a> OptionIntoWasmAbi for &'a _mod_name {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        impl FromWasmAbi for _mod_name {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                _mod_name {
                    obj: JsValue::from_abi(js).into(),
                }
            }
        }
        impl OptionFromWasmAbi for _mod_name {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        impl<'a> IntoWasmAbi for &'a _mod_name {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi {
                (&self.obj).into_abi()
            }
        }
        impl RefFromWasmAbi for _mod_name {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<_mod_name>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(_mod_name {
                    obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
                })
            }
        }
        impl LongRefFromWasmAbi for _mod_name {
            type Abi = <JsValue as LongRefFromWasmAbi>::Abi;
            type Anchor = _mod_name;
            #[inline]
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as LongRefFromWasmAbi>::long_ref_from_abi(js);
                _mod_name { obj: tmp.into() }
            }
        }
        impl From<JsValue> for _mod_name {
            #[inline]
            fn from(obj: JsValue) -> _mod_name {
                _mod_name { obj: obj.into() }
            }
        }
        impl AsRef<JsValue> for _mod_name {
            #[inline]
            fn as_ref(&self) -> &JsValue {
                self.obj.as_ref()
            }
        }
        impl AsRef<_mod_name> for _mod_name {
            #[inline]
            fn as_ref(&self) -> &_mod_name {
                self
            }
        }
        impl From<_mod_name> for JsValue {
            #[inline]
            fn from(obj: _mod_name) -> JsValue {
                obj.obj.into()
            }
        }
        impl JsCast for _mod_name {
            fn instanceof(val: &JsValue) -> bool {
                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                unsafe fn __wbg_instanceof__mod_name_c216b8d85529be3e(_: u32) -> u32 {
                    {
                        ::std::rt::begin_panic(
                            "cannot check instanceof on non-wasm targets",
                        )
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof__mod_name_c216b8d85529be3e(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                _mod_name { obj: val.into() }
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const _mod_name) }
            }
        }
        impl JsObject for _mod_name {}
    };
    #[automatically_derived]
    impl core::ops::Deref for _mod_name {
        type Target = wasm_bindgen::JsValue;
        #[inline]
        fn deref(&self) -> &wasm_bindgen::JsValue {
            &self.obj
        }
    }
    #[automatically_derived]
    impl _mod_name {
        #[allow(nonstandard_style)]
        #[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
        ///This is an internal function, generated by the #[js_bind] macro. By design, the module is private
        fn some_func_name(param: String) -> Result<i32, JsValue> {
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __wbg_someFuncName_4263fcc182a68ed6(
                param: <String as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
                drop(param);
                {
                    ::std::rt::begin_panic(
                        "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
                    )
                };
            }
            unsafe {
                let _ret = {
                    let param = <String as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        param,
                    );
                    __wbg_someFuncName_4263fcc182a68ed6(param)
                };
                wasm_bindgen::__rt::take_last_exception()?;
                Ok(<i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
            }
        }
    }
    pub fn some_func_name(param: String) -> Result<i32, JsValue> {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    format_args!(
                        "Calling function: {0}::{1}(<parameters pass not implemented yet>)",
                        "_mod_name", "some_func_name"
                    ),
                    lvl,
                    &(
                        "firebase_js_sys_proc",
                        "firebase_js_sys_proc",
                        "firebase-js-sys/proc/src/main.rs",
                        6u32,
                    ),
                    ::log::__private_api::Option::None,
                );
            }
        };
        _mod_name::some_func_name(param)
    }
    some_func_name("".to_string()).ok();
    _mod_name::some_func_name("".to_string()).ok();
}
