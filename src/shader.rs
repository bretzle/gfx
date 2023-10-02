use glow::HasContext;

use crate::uniform::{UniformType, UniformBlockLayout};
use std::{error::Error, fmt::Display};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct ShaderId(pub(crate) usize);

#[derive(Debug)]
pub(crate) struct ShaderUniform {
    pub gl_loc: Option<glow::UniformLocation>,
    pub uniform_type: UniformType,
    pub array_count: i32,
}

pub(crate) struct ShaderInternal {
    pub program: glow::Program,
    pub images: Vec<ShaderImage>,
    pub uniforms: Vec<ShaderUniform>,
}

impl ShaderInternal {
    pub fn new(
        gl: &glow::Context,
        shader: ShaderSource<'_>,
        meta: ShaderMeta,
    ) -> Result<ShaderInternal, ShaderError> {
        unsafe {
            let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, shader.vertex)?;
            let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, shader.fragment)?;

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let error = gl.get_program_info_log(program);
                return Err(ShaderError::LinkError(error));
            }

            gl.use_program(Some(program));

            #[rustfmt::skip]
            let images = meta.images.iter().map(|name| ShaderImage {
                gl_loc: gl.get_uniform_location(program, name),
            }).collect();

            #[rustfmt::skip]
            let uniforms = meta.uniforms.uniforms.iter().scan(0, |offset, uniform| {
                let res = ShaderUniform {
                    gl_loc: gl.get_uniform_location(program, &uniform.name),
                    uniform_type: uniform.uniform_type,
                    array_count: uniform.array_count as _,
                };
                *offset += uniform.uniform_type.size() * uniform.array_count;
                Some(res)
            }).collect();

            Ok(ShaderInternal {
                program,
                images,
                uniforms,
            })
        }
    }
}

pub(crate) struct ShaderImage {
    pub gl_loc: Option<glow::UniformLocation>,
}

fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, ShaderError> {
    unsafe {
        let shader = gl.create_shader(shader_type).unwrap();

        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            let error_message = gl.get_shader_info_log(shader);
            return Err(ShaderError::CompilationError {
                shader_type: match shader_type {
                    glow::VERTEX_SHADER => ShaderType::Vertex,
                    glow::FRAGMENT_SHADER => ShaderType::Fragment,
                    _ => unreachable!(),
                },
                error_message,
            });
        }

        Ok(shader)
    }
}

#[derive(Default, Clone)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images: Vec<String>,
}

#[derive(Clone, Debug, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Clone, Debug)]
pub enum ShaderError {
    CompilationError {
        shader_type: ShaderType,
        error_message: String,
    },
    LinkError(String),
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Display the same way as Debug
    }
}

impl Error for ShaderError {}

pub struct ShaderSource<'a> {
    pub vertex: &'a str,
    pub fragment: &'a str,
}
