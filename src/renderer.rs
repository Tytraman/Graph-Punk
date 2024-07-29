use std::cell::{Ref, RefMut};

use gl::types::GLint;
use sdl2::video::GLContext;

use crate::{
    maths::{mat::Mat4, vec::Vec2},
    resource::{renderer_resource::DrawingResource, Resource},
};

pub mod data_object;
pub mod draw;
pub mod uniform;
pub mod vao;
pub mod vbo;

#[macro_export]
macro_rules! gl_exec {
    ( $closure:expr ) => {{
        unsafe {
            clear_errors();

            $closure();

            check_errors(stringify!($closure))
        }
    }};
}

pub struct Renderer {
    context: GLContext,
    display_size: Vec2<i32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
    pub(crate) projection: Mat4<f32>,
}

impl Renderer {
    pub fn new(context: GLContext, display_size: Vec2<i32>) -> Self {
        Self {
            context,
            display_size,
            aspect_ratio: 1.0_f32,
            left: 0.0_f32,
            bottom: 0.0_f32,
            projection: Mat4::default(),
        }
    }

    pub fn set_display_size(&mut self, display_size: Vec2<i32>) {
        self.display_size = display_size;
    }

    pub fn get_display_size(&self) -> Vec2<i32> {
        self.display_size.clone()
    }

    pub fn get_pixel<'a>(
        &'a mut self,
        resource: &'a Resource,
        x: usize,
        y: usize,
    ) -> Result<Ref<'_, DrawingResource>, String> {
        if x >= self.display_size.x as usize || y >= self.display_size.y as usize {
            return Err("indexes are out of bound".to_string());
        }

        let mut drawing_objects = resource
            .query::<DrawingResource>()
            .ok_or("no drawing objects".to_string())?;

        let index = y * self.display_size.x as usize + x;

        // `swap_remove` permet de récupérer l'ownership de la valeur retirée.
        let pixel = drawing_objects.swap_remove(index);

        Ok(pixel)
    }

    pub fn get_pixel_mut<'a>(
        &'a mut self,
        resource: &'a mut Resource,
        x: usize,
        y: usize,
    ) -> Result<RefMut<'_, DrawingResource>, String> {
        if x >= self.display_size.x as usize || y >= self.display_size.y as usize {
            return Err("indexes are out of bound".to_string());
        }

        let mut drawing_objects = resource
            .query_mut::<DrawingResource>()
            .ok_or("no drawing objects".to_string())?;

        let index = y * self.display_size.x as usize + x;

        // `swap_remove` permet de récupérer l'ownership de la valeur retirée.
        let pixel = drawing_objects.swap_remove(index);

        Ok(pixel)
    }

    pub fn clear_grid_pixel(&mut self, resource: &mut Resource) -> Result<(), String> {
        for x in 0..self.display_size.x as usize {
            for y in 0..self.display_size.y as usize {
                self.set_grid_pixel(resource, x, y, false)?;
            }
        }

        Ok(())
    }

    pub fn toggle_grid_pixel(
        &mut self,
        resource: &mut Resource,
        x: usize,
        y: usize,
    ) -> Result<(), String> {
        let mut pixel = match self.get_pixel_mut(resource, x, y) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let visible = pixel.0.is_visible();

        pixel.0.set_visible(!visible);

        Ok(())
    }

    pub fn set_grid_pixel(
        &mut self,
        resource: &mut Resource,
        x: usize,
        y: usize,
        value: bool,
    ) -> Result<(), String> {
        let mut pixel = match self.get_pixel_mut(resource, x, y) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        pixel.0.set_visible(value);

        Ok(())
    }

    pub fn set_viewport_size(&self, width: i32, height: i32) -> Result<(), String> {
        gl_exec!(|| gl::Viewport(0, 0, width as GLint, height as GLint))
    }

    pub fn borrow_context(&self) -> &GLContext {
        &self.context
    }
}

pub fn clear_errors() {
    unsafe {
        loop {
            if gl::GetError() == gl::NO_ERROR {
                break;
            }
        }
    }
}

pub fn check_errors(function_name: &str) -> Result<(), String> {
    let mut message = String::new();
    let mut code;

    unsafe {
        loop {
            let error = gl::GetError();

            if error == gl::NO_ERROR {
                break;
            }

            code = stringify!(error);

            message.push_str(&format!("[OpenGL error] {function_name}: {code}"));
        }
    }

    if message.len() > 0 {
        return Err(message);
    }

    Ok(())
}
