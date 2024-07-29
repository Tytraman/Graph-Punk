use crate::{
    maths::{
        mat::Mat4,
        vec::{Vec3, Vec4},
    },
    renderer::{
        data_object::{AttribPointer, DataObject},
        draw::Draw,
        uniform::Uniform,
        Renderer,
    },
    resource::Resource,
    shader::program::ShaderProgram,
};

use std::mem;

#[derive(Clone)]
pub struct Rectangle {
    data_object: DataObject,
}

impl Rectangle {
    pub fn build(
        resource: &mut Resource,
        shader_program: ShaderProgram,
        color: Vec4<f32>,
        position: Vec3<f32>,
        size: Vec3<f32>,
    ) -> Result<Self, String> {
        let x = 0.5_f32;
        let y = 0.5_f32;

        let pixel = vec![
            -x, -y, 0.0_f32, x, -y, 0.0_f32, -x, y, 0.0_f32, x, -y, 0.0_f32, -x, y, 0.0_f32, x, y,
            0.0_f32,
        ];

        let attrib_pointer = AttribPointer {
            index: 0,
            size: 3,
            stride: (3 * mem::size_of::<f32>()) as i32,
            offset: 0,
        };

        let mut rect = match DataObject::build(
            resource,
            pixel,
            &vec![attrib_pointer],
            color.clone(),
            position,
            size,
        ) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let uniforms = rect.borrow_mut_uniforms();

        if let Err(err) = shader_program.use_it() {
            return Err(err);
        }

        let color_name = "punk_color";
        let mut color_uniform = Uniform::new();
        let mut model_uniform = Uniform::new();
        let mut projection_uniform = Uniform::new();

        if let Err(err) = color_uniform.search(&shader_program, color_name) {
            return Err(err);
        }

        let punk_model = "punk_model";
        if let Err(err) = model_uniform.search(&shader_program, punk_model) {
            return Err(err);
        }

        let punk_projection = "punk_projection";
        if let Err(err) = projection_uniform.search(&shader_program, punk_projection) {
            return Err(err);
        }

        if let Err(err) = color_uniform.send_vec4(&color) {
            return Err(err);
        }

        uniforms.insert(color_name.to_string(), color_uniform);
        uniforms.insert(punk_model.to_string(), model_uniform);
        uniforms.insert(punk_projection.to_string(), projection_uniform);

        rect.set_shader_program(shader_program);

        Ok(Self { data_object: rect })
    }
}

impl Draw for Rectangle {
    fn draw(&self, renderer: &Renderer, projection: &Mat4<f32>) -> Result<(), String> {
        self.data_object.draw(renderer, projection)
    }

    fn get_color(&self) -> Vec4<f32> {
        self.data_object.get_color()
    }

    fn set_color(&mut self, color: Vec4<f32>) {
        self.data_object.set_color(color);
    }

    fn get_position(&self) -> Vec3<f32> {
        self.data_object.get_position()
    }

    fn get_scale(&self) -> Vec3<f32> {
        self.data_object.get_scale()
    }

    fn set_position(&mut self, position: Vec3<f32>) {
        self.data_object.set_position(position);
    }

    fn set_scale(&mut self, scale: Vec3<f32>) {
        self.data_object.set_scale(scale);
    }

    fn is_visible(&self) -> bool {
        self.data_object.is_visible()
    }

    fn set_visible(&mut self, value: bool) {
        self.data_object.set_visible(value);
    }
}
