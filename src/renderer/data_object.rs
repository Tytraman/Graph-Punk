use std::collections::HashMap;

use gl::types::GLsizei;

use crate::{
    gl_exec,
    maths::{
        mat::Mat4,
        vec::{Vec3, Vec4},
    },
    resource::{gl_resource::VBOResource, Resource},
    shader::program::ShaderProgram,
};

use super::{check_errors, clear_errors, uniform::Uniform, vao::VAO, vbo::VBO, Renderer};

#[derive(Clone)]
pub struct DataObject {
    vertices_number: usize,
    vao: VAO,
    shader_program: ShaderProgram,
    uniforms: HashMap<String, Uniform>,
    color: Vec4<f32>,
    position: Vec3<f32>,
    scale: Vec3<f32>,
    visible: bool,
}

pub struct AttribPointer {
    pub index: u32,
    pub size: i32,
    pub stride: i32,
    pub offset: usize,
}

pub type AttribPointers = Vec<AttribPointer>;

impl DataObject {
    pub fn build(
        resource: &mut Resource,
        vertices: Vec<f32>,
        attrib_pointers: &AttribPointers,
        color: Vec4<f32>,
        position: Vec3<f32>,
        scale: Vec3<f32>,
    ) -> Result<Self, String> {
        let vao = match VAO::build() {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        if let Err(err) = vao.bind() {
            return Err(err);
        }

        let vbo = match VBO::build(vertices) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let vertices_number = vbo.borrow_vertices().len();

        let vbo_resource = VBOResource(vbo);
        resource.add(&vbo_resource.0.get_id().to_string(), vbo_resource);

        let mut index = 0;
        for attrib in attrib_pointers.iter() {
            if let Err(err) =
                vao.attrib_pointer(attrib.index, attrib.size, attrib.stride, attrib.offset)
            {
                return Err(err);
            };

            if let Err(err) = vao.enable_attrib(index) {
                return Err(err);
            }

            index += 1;
        }

        Ok(Self {
            vertices_number,
            vao,
            shader_program: ShaderProgram::none(),
            uniforms: HashMap::new(),
            color,
            position,
            scale,
            visible: true,
        })
    }

    pub fn draw(&self, renderer: &Renderer, projection: &Mat4<f32>) -> Result<(), String> {
        let punk_model = "punk_model";
        let punk_projection = "punk_projection";
        let punk_color = "punk_color";

        let mut model = Mat4::default();

        let model_uniform = match self.uniforms.get(punk_model) {
            Some(t) => t,
            None => return Err(format!("{punk_model} uniform not found")),
        };

        let projection_uniform = match self.uniforms.get(punk_projection) {
            Some(t) => t,
            None => return Err(format!("{punk_projection} uniform not found")),
        };

        let color_uniform = match self.uniforms.get(punk_color) {
            Some(t) => t,
            None => return Err(format!("{punk_color} uniform not found")),
        };

        if let Err(err) = self.shader_program.use_it() {
            return Err(err);
        }

        let mut position = self.position.clone();

        let display_size = renderer.get_display_size();

        // Fait en sorte que l'origine soit en haut à gauche du rendu.
        position.x -= display_size.x as f32 * 0.5_f32;
        position.y -= display_size.y as f32 * 0.5_f32;

        // TODO: rotate
        model = Mat4::translate(&model, &position);
        model = Mat4::scale(&model, &self.scale);

        if let Err(err) = model_uniform.send_mat4(&model) {
            return Err(err);
        }

        if let Err(err) = projection_uniform.send_mat4(&projection) {
            return Err(err);
        }

        if let Err(err) = color_uniform.send_vec4(&self.color) {
            return Err(err);
        }

        if let Err(err) = self.vao.bind() {
            return Err(err);
        }

        gl_exec!(|| gl::DrawArrays(gl::TRIANGLES, 0, self.get_vertices_number() as GLsizei))
    }

    pub fn get_vertices_number(&self) -> usize {
        self.vertices_number
    }

    pub fn get_color(&self) -> Vec4<f32> {
        self.color.clone()
    }

    pub fn set_color(&mut self, color: Vec4<f32>) {
        self.color = color;
    }

    pub fn get_position(&self) -> Vec3<f32> {
        self.position.clone()
    }

    pub fn get_scale(&self) -> Vec3<f32> {
        self.scale.clone()
    }

    pub fn set_position(&mut self, position: Vec3<f32>) {
        self.position = position;
    }

    pub fn set_scale(&mut self, scale: Vec3<f32>) {
        self.scale = scale;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn borrow_vao(&self) -> &VAO {
        &self.vao
    }

    pub fn borrow_shader_program(&self) -> &ShaderProgram {
        &self.shader_program
    }

    pub fn borrow_uniforms(&self) -> &HashMap<String, Uniform> {
        &self.uniforms
    }

    pub fn borrow_mut_uniforms(&mut self) -> &mut HashMap<String, Uniform> {
        &mut self.uniforms
    }

    pub fn set_shader_program(&mut self, shader_program: ShaderProgram) {
        self.shader_program = shader_program;
    }
}
