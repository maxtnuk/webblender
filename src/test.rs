use wasm_bindgen::prelude::*;
// this mod for testing
#[wasm_bindgen]
pub fn gl_init() {
    use crate::gl;
    let canvas = gl::get_canvas("canvas").unwrap();
}
