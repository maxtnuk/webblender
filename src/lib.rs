use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};
use std::rc::Rc;

pub mod prelude{
    pub use crate::app::App;
}
use crate::prelude::*;

pub mod util;
pub mod app;
pub mod shader;
pub mod frag;

pub fn get_canvas(id: &str) -> Result<WebGl2RenderingContext, js_sys::Object> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct WebGlView {
    app: Rc<App>,
    gl: Rc<WebGl2RenderingContext>,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl WebGlView {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        WebGlView {
            app: Rc::new(App::new()),
            gl: Rc::new(gl),
            program: None
        }
    }
    #[allow(dead_code)]
    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            log!(
                "error {}",
                context
                    .get_shader_info_log(&shader)
                    .unwrap_or("".to_string())
            );
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
    #[allow(dead_code)]
    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
    pub fn start(&self) {

    }
    pub fn update(&self){

    }
    pub fn render(&self){

    }
}
