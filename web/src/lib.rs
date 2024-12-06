mod utils;

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub mod c_shim;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    dapper::dapper_hey(); // purely to call into something that dep's on C.
    alert("Hello, hello-wasm!");
    alert(&dapper::treedemo());
}
