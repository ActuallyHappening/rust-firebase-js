#[cfg(test)] #[:: wasm_bindgen_test :: wasm_bindgen_test] fn
_gentest_test_initialize_app()
{
    use wasm_bindgen :: JsValue ; #[cfg(feature = "web-not-node")] ::
    wasm_bindgen_test :: wasm_bindgen_test_configure! (run_in_browser) ;
    assert!
    (initialize_app(JsValue :: UNDEFINED, JsValue :: UNDEFINED).is_err()) ;
    assert! (initialize_app(JsValue :: NULL, JsValue :: UNDEFINED).is_err()) ;
    assert!
    (initialize_app(serde_wasm_bindgen ::
    to_value(& serde_json :: json! ({})).unwrap(), JsValue ::
    UNDEFINED).is_ok()) ; assert!
    (initialize_app(serde_wasm_bindgen ::
    to_value(& serde_json :: json! ({})).unwrap(), JsValue ::
    from_str("project name here")).is_ok()) ;
}