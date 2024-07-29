use crate::maths::{
    mat::Mat4,
    vec::{Vec3, Vec4},
};

use super::Renderer;

pub trait Draw {
    fn draw(&self, renderer: &Renderer, projection: &Mat4<f32>) -> Result<(), String>;

    fn get_color(&self) -> Vec4<f32>;
    fn set_color(&mut self, color: Vec4<f32>);

    fn get_position(&self) -> Vec3<f32>;
    fn set_position(&mut self, position: Vec3<f32>);

    fn get_scale(&self) -> Vec3<f32>;
    fn set_scale(&mut self, scale: Vec3<f32>);

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, value: bool);
}
