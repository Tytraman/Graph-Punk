use std::collections::HashMap;

use gl::types::GLsizei;

use crate::{
    gl_exec,
    maths::{
        mat::Mat4,
        vec::{Vec3, Vec4},
    },
    shader::program::ShaderProgram,
};

use super::{
    uniform::Uniform,
    vao::VAO,
    vbo::{VBOType, VBO},
    Renderer,
};

#[derive(Clone)]
pub struct DataObject {
    pub(crate) vertices_number: usize,
    pub(crate) vao: VAO,
    pub(crate) vbo: VBO,
    pub(crate) shader_program: ShaderProgram,
    pub(crate) uniforms: HashMap<String, Uniform>,
    pub(crate) color: Vec4<f32>,
    pub(crate) position: Vec3<f32>,
    pub(crate) scale: Vec3<f32>,
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
        vertices: Vec<f32>,
        size: isize,
        attrib_pointers: &AttribPointers,
        color: Vec4<f32>,
        position: Vec3<f32>,
        scale: Vec3<f32>,
        type_: VBOType,
    ) -> Result<Self, String> {
        let vao = VAO::build()?;

        vao.bind()?;

        let vbo = VBO::build(vertices, size, type_)?;

        let vertices_number = vbo.borrow_vertices().len();

        let mut index = 0;
        for attrib in attrib_pointers.iter() {
            vao.attrib_pointer(attrib.index, attrib.size, attrib.stride, attrib.offset)?;
            vao.enable_attrib(index)?;

            index += 1;
        }

        Ok(Self {
            vertices_number,
            vao,
            vbo,
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

        let model_uniform = self
            .uniforms
            .get(punk_model)
            .ok_or(format!("{punk_model} uniform not found"))?;

        let projection_uniform = self
            .uniforms
            .get(punk_projection)
            .ok_or(format!("{punk_projection} uniform not found"))?;

        let color_uniform = self
            .uniforms
            .get(punk_color)
            .ok_or(format!("{punk_color} uniform not found"))?;

        self.shader_program.use_it()?;

        let mut position = self.position.clone();

        let display_size = renderer.get_display_size();

        // Fait en sorte que l'origine soit en haut à gauche du rendu.
        position.x -= display_size.x as f32 * 0.5_f32;
        position.y -= display_size.y as f32 * 0.5_f32;

        // TODO: rotate
        model = Mat4::translate(&model, &position);
        model = Mat4::scale(&model, &self.scale);

        model_uniform.send_mat4(&model)?;
        projection_uniform.send_mat4(&projection)?;
        color_uniform.send_vec4(&self.color)?;

        self.vao.bind()?;

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
