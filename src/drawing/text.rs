use std::{mem, os};

use crate::{
    gl_exec,
    maths::vec::{Vec3, Vec4},
    renderer::{
        data_object::{AttribPointer, DataObject},
        draw::Draw,
        uniform::Uniform,
        vbo::VBOType,
    },
    shader::program::ShaderProgram,
};

#[derive(Clone)]
pub struct Text {
    data_object: DataObject,
    text: String,
}

impl Text {
    pub fn build(
        shader_program: ShaderProgram,
        color: Vec4<f32>,
        position: Vec3<f32>,
        text: &str,
    ) -> Result<Self, String> {
        let vertices_size = mem::size_of::<f32>() * 6 * 4;
        let attrib_pointer = AttribPointer {
            index: 0,
            size: 4,
            stride: (4 * mem::size_of::<f32>()) as i32,
            offset: 0,
        };

        let mut data_object = DataObject::build(
            Vec::new(),
            vertices_size as isize,
            &vec![attrib_pointer],
            color.clone(),
            position,
            Vec3 {
                x: 1.0_f32,
                y: 1.0_f32,
                z: 1.0_f32,
            },
            VBOType::DynamicDraw,
        )?;

        let uniforms = data_object.borrow_mut_uniforms();

        shader_program.use_it()?;

        let texture_name = "punk_texture";
        let texture_color_name = "punk_texture_color";
        let projection_name = "punk_projection";

        let mut texture_uniform = Uniform::new();
        let mut texture_color_uniform = Uniform::new();
        let mut projection_uniform = Uniform::new();

        texture_uniform.search(&shader_program, texture_name)?;
        texture_color_uniform.search(&shader_program, texture_color_name)?;
        projection_uniform.search(&shader_program, projection_name)?;

        uniforms.insert(texture_name.to_string(), texture_uniform);
        uniforms.insert(texture_color_name.to_string(), texture_color_uniform);
        uniforms.insert(projection_name.to_string(), projection_uniform);

        data_object.set_shader_program(shader_program);

        Ok(Self {
            data_object,
            text: text.to_string(),
        })
    }
}

impl Draw for Text {
    fn draw(
        &self,
        renderer: &crate::renderer::Renderer,
        projection: &crate::maths::mat::Mat4<f32>,
    ) -> Result<(), String> {
        // Dessiner un texte ne se fait pas de la même manière que les autres objets.
        // Ici chaque lettre va être rendue une par une en modifiant le contenu du buffer OpenGL.

        let punk_texture_color = "punk_texture_color";
        let punk_projection = "punk_projection";

        let texture_color_uniform = self
            .data_object
            .uniforms
            .get(punk_texture_color)
            .ok_or(format!("{punk_texture_color} uniform not found"))?;

        let projection_uniform = self
            .data_object
            .uniforms
            .get(punk_projection)
            .ok_or(format!("{punk_projection} uniform not found"))?;

        self.data_object.shader_program.use_it()?;

        let mut position = self.data_object.position.clone();

        let display_size = renderer.get_display_size();

        // Fait en sorte que l'origine soit en haut à gauche du rendu.
        position.x -= display_size.x as f32 * 0.5_f32;
        position.y -= display_size.y as f32 * 0.5_f32;

        let color = self.data_object.color.clone();
        let color = Vec3 {
            x: color.x,
            y: color.y,
            z: color.z,
        };

        texture_color_uniform.send_vec3(&color)?;
        projection_uniform.send_mat4(&projection)?;

        gl_exec!(|| gl::ActiveTexture(gl::TEXTURE0))?;

        self.data_object.vao.bind()?;

        // Itère à travers toutes les lettres du texte.
        for c in self.text.chars() {
            let charactere = renderer
                .font_context
                .characteres
                .get(&c)
                .ok_or(format!("cannot get font for {c} charactere"))?;

            let xpos = position.x + charactere.bearing.x as f32 * self.data_object.scale.x;
            let ypos = position.y
                + (charactere.size.y - charactere.bearing.y as u32) as f32
                    * self.data_object.scale.y;

            let w = charactere.size.x as f32 * self.data_object.scale.x;
            let h = charactere.size.y as f32 * self.data_object.scale.y;

            let vertices = [
                xpos,
                ypos - h,
                0.0_f32,
                0.0_f32,
                xpos,
                ypos,
                0.0_f32,
                1.0_f32,
                xpos + w,
                ypos,
                1.0_f32,
                1.0_f32,
                xpos,
                ypos - h,
                0.0_f32,
                0.0_f32,
                xpos + w,
                ypos,
                1.0_f32,
                1.0_f32,
                xpos + w,
                ypos - h,
                1.0_f32,
                0.0_f32,
            ];

            // Fait le rendu du caractère.
            gl_exec!(|| gl::BindTexture(gl::TEXTURE_2D, charactere.texture_id))?;

            // Met à jour le contenu du buffer OpenGL.
            self.data_object.vbo.bind()?;
            gl_exec!(|| gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (mem::size_of::<f32>() * 24) as isize,
                vertices.as_ptr() as *const os::raw::c_void
            ))?;

            // Fait le rendu.
            gl_exec!(|| gl::DrawArrays(gl::TRIANGLES, 0, 6))?;

            // Avance le curseur.
            position.x += (charactere.advance_x >> 6) as f32;
        }

        gl_exec!(|| gl::BindVertexArray(0))?;
        gl_exec!(|| gl::BindTexture(gl::TEXTURE_2D, 0))?;

        Ok(())
    }

    fn get_color(&self) -> Vec4<f32> {
        self.data_object.get_color()
    }

    fn set_color(&mut self, color: Vec4<f32>) {
        self.data_object.set_color(color);
    }

    fn get_scale(&self) -> Vec3<f32> {
        self.data_object.get_scale()
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

    fn get_position(&self) -> Vec3<f32> {
        self.data_object.get_position()
    }

    fn set_position(&mut self, position: Vec3<f32>) {
        self.data_object.set_position(position);
    }
}
