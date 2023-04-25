use wasm_bindgen::prelude::Closure;

type closure<Args> = Closure<dyn FnMut(Args)>;

pub mod app;

pub mod database;