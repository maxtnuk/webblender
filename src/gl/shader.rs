use crate::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
use std::borrow::BorrowMut;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};
use std::collections::HashMap;

type SourceHash = HashMap<String,(String,String)>;
type ShaderHash = HashMap<String,Shader>;

pub struct ShaderController{
    gl: WebGl2RenderingContext,
    sources: SourceHash,
    programs: ShaderHash
}
impl ShaderController{
    pub fn new(gl: WebGl2RenderingContext) -> Self{
        ShaderController{
            gl: gl,
            sources: HashMap::new(),
            programs: HashMap::new()
        }
    }
    pub fn init_source<F>(&mut self,mut source_init: F)
    where F: FnMut(&mut SourceHash){
        source_init(self.sources.borrow_mut());

        for (name,source) in self.sources.iter(){
            let shader= Shader::new(&self.gl,source.clone());
            self.programs.insert(name.to_string(),shader);
        }
    }
}
enum ShaderType{
    Vertex,
    Fragment,
    Uniform,
    Texture,
}

struct Shader{
    pub program: WebGlProgram,
}
impl Shader{
    //(vertex,frag)
    fn new(gl: &WebGl2RenderingContext,source: (String,String)) -> Self{
        let vs=compile_shader(gl,WebGl2RenderingContext::VERTEX_SHADER,source.0.as_str()).unwrap();
        let fs=compile_shader(gl,WebGl2RenderingContext::FRAGMENT_SHADER,source.1.as_str()).unwrap();

        let program = link_program(gl,&vs,&fs).unwrap();
        gl.use_program(Some(&program));
        Shader{
            program: program.clone()
        }
    }
    pub fn init_buffer(&self,gl: &WebGl2RenderingContext,draw_type: u32,source: Vec<f32>) -> Result<(), JsValue> {

        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();

        let source_location = source.as_ptr() as u32 / 4;
        let source_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(source_location, source_location + source.len() as u32);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &source_array,
            draw_type//WebGl2RenderingContext::STATIC_DRAW,
        );
        Ok(())
    }
    pub fn attrib_data_form(&self,gl: &WebGl2RenderingContext,shader_type: ShaderType,attrib_name: &str, size: i32) -> Result<(), JsValue> {
        match shader_type{
            ShaderType::Vertex =>{
                let va = gl.create_vertex_array();
                gl.bind_vertex_array(va.as_ref()); // this is only for vertex

                let position_att_location = gl
            .get_attrib_location(&self.program, attrib_name) as u32;

                gl.enable_vertex_attrib_array(position_att_location);
                gl.vertex_attrib_pointer_with_i32(
                    position_att_location,
                    size,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
            }
            _ => {
                let attrib_location = gl
            .get_attrib_location(&self.program, attrib_name) as u32;
                gl.enable_vertex_attrib_array(attrib_location);

                gl.vertex_attrib_pointer_with_i32(
                    attrib_location,
                    size,
                    WebGl2RenderingContext::FLOAT,
                    false,
                    0,
                    0,
                );
            }
        }
        Ok(())
    }
}


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
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

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
