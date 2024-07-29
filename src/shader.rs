use std::{ffi::CString, ptr};

use crate::common::PunkString;
use crate::gl_exec;
use crate::renderer::{check_errors, clear_errors};

pub mod program;

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    shader_type: ShaderType,
    name: String,
    source: String,
    id: u32,
}

impl Shader {
    pub fn new(shader_type: ShaderType, name: String, source: String) -> Self {
        Self {
            shader_type,
            name,
            source,
            id: 0,
        }
    }

    pub fn create(&mut self) -> Result<(), String> {
        let t = match self.shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        };

        gl_exec!(|| self.id = gl::CreateShader(t))
    }

    pub fn source(&self) -> Result<(), String> {
        let c_str = match CString::new(self.source.as_bytes()) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        gl_exec!(|| gl::ShaderSource(self.id, 1, &c_str.as_ptr(), ptr::null()))
    }

    pub fn compile(&self) -> Result<(), String> {
        if let Err(err) = gl_exec!(|| gl::CompileShader(self.id)) {
            return Err(err);
        }

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        // S'il y a une erreur, 'success' vaudra 0.
        if let Err(err) = gl_exec!(|| gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success)) {
            return Err(err);
        }

        if success == 0 {
            if let Err(err) = gl_exec!(|| gl::GetShaderInfoLog(
                self.id,
                info_log.len() as i32,
                ptr::null_mut(),
                info_log.as_mut_ptr(),
            )) {
                return Err(err);
            }

            return Err(PunkString::i8_array_to_string(&info_log));
        }

        Ok(())
    }

    pub fn destroy(&self) -> Result<(), String> {
        gl_exec!(|| gl::DeleteShader(self.id))
    }

    pub fn is_vertex_shader(&self) -> bool {
        matches!(self.shader_type, ShaderType::Vertex)
    }

    pub fn is_fragment_shader(&self) -> bool {
        matches!(self.shader_type, ShaderType::Fragment)
    }

    pub fn borrow_name(&self) -> &String {
        &self.name
    }
}
