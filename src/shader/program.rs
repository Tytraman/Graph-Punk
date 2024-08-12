use std::ptr;

use gl::types::GLuint;

use crate::common::PunkString;
use crate::gl_exec;

use super::Shader;

#[derive(Clone)]
pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn none() -> Self {
        Self { id: 0 }
    }

    pub fn build(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, String> {
        if !vertex_shader.is_vertex_shader() {
            return Err("'vertex_shader' is not a Vertex Shader".to_string());
        }

        if !fragment_shader.is_fragment_shader() {
            return Err("'fragment_shader' is not a Fragment Shader".to_string());
        }

        let mut id = 0;

        if let Err(err) = gl_exec!(|| id = gl::CreateProgram()) {
            return Err(err);
        }

        if let Err(err) = gl_exec!(|| gl::AttachShader(id, vertex_shader.id)) {
            return Err(err);
        }

        if let Err(err) = gl_exec!(|| gl::AttachShader(id, fragment_shader.id)) {
            return Err(err);
        }

        Ok(ShaderProgram { id })
    }

    pub fn link(&self) -> Result<(), String> {
        if let Err(err) = gl_exec!(|| gl::LinkProgram(self.id)) {
            return Err(err);
        }

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        if let Err(err) = gl_exec!(|| gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success)) {
            return Err(err);
        }

        if success == 0 {
            if let Err(err) = gl_exec!(|| gl::GetProgramInfoLog(
                self.id,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr()
            )) {
                return Err(err);
            }

            return Err(PunkString::i8_array_to_string(&info_log));
        }

        Ok(())
    }

    pub fn use_it(&self) -> Result<(), String> {
        gl_exec!(|| gl::UseProgram(self.id))
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }
}
