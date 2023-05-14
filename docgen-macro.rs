#[cfg_attr(feature = "web-not-node", :: wasm_bindgen :: prelude ::
wasm_bindgen(module = "/js/bundle-esm.js"))]
#[cfg_attr(feature = "node-not-web", :: wasm_bindgen :: prelude ::
wasm_bindgen(module = "/js/bundle-cjs.js"))] extern "C"
{
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
    /// ## Examples
    /// ```rust
    /// use firebase_js_sys::app;
    /// use wasm_bindgen::JsValue;
    /// 
    /// let config = JsValue::UNDEFINED;
    /// let returned = app::initialize_app(config);
    /// 
    /// assert!(returned.is_err());
    /// ```
    #[wasm_bindgen(js_name = "initializeApp", catch)] pub fn
    initialize_app(config : JsValue) -> Result < JsValue, JsValue > ;
}