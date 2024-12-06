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
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // panic!("this better be smart");
    dapper::dapper_hey(); // purely to call into something that dep's on C.
    alert("Hello, hello-wasm!");
    alert(&dapper::treedemo());
}
