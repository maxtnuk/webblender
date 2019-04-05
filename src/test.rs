use wasm_bindgen::prelude::*;
// this mod for testing
#[wasm_bindgen]
pub fn gl_init() {
    use crate::gl;
    use crate::gl::shader;

    let canvas = gl::get_canvas("canvas").unwrap();
    let mut shader_con= shader::ShaderController::new(canvas);

    shader_con.init_source(|x|{
        let triangle_source = (include_str!("../source/vert/triangle.glsl").to_string(),
                                include_str!("../source/frag/triangle.glsl").to_string());
        x.insert("triangle".to_string(),triangle_source);
    })
}
