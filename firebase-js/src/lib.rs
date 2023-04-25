use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

type closure<Args> = Closure<dyn FnMut(Args)>;

pub mod app;

pub mod database;