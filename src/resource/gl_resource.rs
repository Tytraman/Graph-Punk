use crate::{
    renderer::{vao::VAO, vbo::VBO},
    shader::program::ShaderProgram,
};

pub struct VBOResource(pub VBO);
pub struct VAOResource(pub VAO);
pub struct ShaderProgramResource(pub ShaderProgram);
