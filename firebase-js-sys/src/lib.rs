use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue};

type closure<Args> = Closure<dyn FnMut(Args)>;

pub mod app;
pub mod database;