# JS Bind
High level **compile time** binding for JavaScript, using `wasm_bindgen` *runtime* linking.

## Usage
```rust
use js_bind::js_bind;
use wasm_bindgen::prelude::*;

#[js_bind(module = "test/sub", doc, test)]