use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};
use std::rc::Rc;

pub mod prelude{
    pub use crate::app::App;
    pub use crate::util::*;
}
use crate::prelude::*;

pub mod gl;
#[macro_use]
pub mod util;
pub mod app;

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
    pub fn start(&self) {

    }
    pub fn update(&self){

    }
    pub fn render(&self){

    }
}
