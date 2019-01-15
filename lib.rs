use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().unwrap();
    window.alert_with_message("test").unwrap();
}
