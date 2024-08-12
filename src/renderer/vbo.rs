use std::ptr;

use gl::types::GLuint;

use crate::gl_exec;

#[derive(Clone)]
pub struct VBO {
    vertices: Vec<f32>,
    id: GLuint,
}

pub enum VBOType {
    StaticDraw,
    DynamicDraw,
}

impl VBO {
    pub fn build(mut vertices: Vec<f32>, size: isize, type_: VBOType) -> Result<Self, String> {
        let mut id = 0;

        // Génère un nouveau VBO.
        gl_exec!(|| gl::GenBuffers(1, &mut id))?;

        // Rend le nouveau VBO actif.
        gl_exec!(|| gl::BindBuffer(gl::ARRAY_BUFFER, id))?;

        // Copie les données dans le VBO nouvellement créé.
        gl_exec!(|| gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            if !vertices.is_empty() {
                vertices.as_mut_ptr() as *const std::os::raw::c_void
            } else {
                ptr::null_mut()
            },
            match type_ {
                VBOType::StaticDraw => gl::STATIC_DRAW,
                VBOType::DynamicDraw => gl::DYNAMIC_DRAW,
            }
        ))?;

        Ok(Self { vertices, id })
    }

    pub fn bind(&self) -> Result<(), String> {
        gl_exec!(|| gl::BindBuffer(gl::ARRAY_BUFFER, self.id))
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn borrow_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
}
