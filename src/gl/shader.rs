use crate::prelude::*;
use std::borrow::BorrowMut;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};
use std::collections::HashMap;

type SourceHash = HashMap<String,(String,String)>;
type ShaderHash = HashMap<String,Shader>;

struct ShaderController{
    sources: SourceHash,
    programs: ShaderHash
}
impl ShaderController{
    fn new() -> Self{
        ShaderController{
            sources: HashMap::new(),
            programs: HashMap::new()
        }
    }
    fn init_source<F>(&mut self,gl: &WebGl2RenderingContext,mut source_init: F)
    where F: FnMut(&mut SourceHash){
        source_init(self.sources.borrow_mut());

        for (name,source) in self.sources.iter(){
            let shader= Shader::new(gl,source.clone());
            self.programs.insert(name.to_string(),shader);
        }
    }
}

struct Shader{
    pub program: WebGlProgram,
}
impl Shader{
    //(vertex,frag)
    fn new(gl: &WebGl2RenderingContext,source: (String,String)) -> Self{

        let vs=compile_shader(gl,WebGl2RenderingContext::VERTEX_SHADER,source.0.as_str()).unwrap();
        let fs=compile_shader(gl,WebGl2RenderingContext::FRAGMENT_SHADER,source.1.as_str()).unwrap();

        Shader{
            program: link_program(gl,&vs,&fs).unwrap()
        }
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
