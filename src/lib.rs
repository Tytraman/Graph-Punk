use std::{
    any::Any,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use benchmark::BenchmarkManager;
use maths::vec::Vec2;
use message::MessageCaller;
use renderer::draw::Draw;
use resource::Resource;
use shader::program::ShaderProgram;
use types::UserData;
use window::{user_input::Keys, window::Window};

pub mod benchmark;
pub mod common;
pub mod drawing;
pub mod maths;
pub mod message;
pub mod renderer;
pub mod resource;
pub mod shader;
pub mod types;
pub mod window;

pub struct GraphPunk<'a> {
    windows: HashMap<String, Window<'a>>,
    resources: Resource,
    drawing_objects: Vec<Box<dyn Draw>>,
    shaders_program: HashMap<String, ShaderProgram>,
    benchmark: BenchmarkManager,
}

impl<'a> GraphPunk<'a> {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            resources: Resource::new(),
            drawing_objects: Vec::new(),
            shaders_program: HashMap::new(),
            benchmark: BenchmarkManager::default(),
        }
    }

    pub fn create_window(
        &mut self,
        unique_id: &str,
        window_title: &str,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let window = Window::new(
            window_title,
            width,
            height,
            Vec2 {
                x: width as i32,
                y: height as i32,
            },
        )?;

        self.windows.insert(unique_id.to_string(), window);

        Ok(())
    }

    pub fn window_set_display_size(
        &mut self,
        unique_id: &str,
        size: Vec2<i32>,
    ) -> Result<(), String> {
        let window = self
            .windows
            .get_mut(unique_id)
            .ok_or("no window found".to_string())?;

        window.set_display_size(size);

        Ok(())
    }

    pub fn window_set_update_callback(
        &mut self,
        unique_id: &str,
        c: impl FnMut(&Keys, &mut UserData) + 'a,
        user_data: UserData,
    ) -> Result<(), String> {
        let window = self
            .windows
            .get_mut(unique_id)
            .ok_or("no window found".to_string())?;

        window.set_update_callback(c, user_data);

        Ok(())
    }

    pub fn window_clear_grid_pixel(&mut self, unique_id: &str) -> Result<(), String> {
        let window = self
            .windows
            .get_mut(unique_id)
            .ok_or("no window found".to_string())?;

        window
            .borrow_renderer_mut()
            .clear_grid_pixel(&mut self.drawing_objects)?;

        Ok(())
    }

    pub fn window_set_grid_pixel(
        &mut self,
        unique_id: &str,
        x: usize,
        y: usize,
        value: bool,
    ) -> Result<(), String> {
        let window = self
            .windows
            .get_mut(unique_id)
            .ok_or("no window found".to_string())?;

        window
            .borrow_renderer_mut()
            .set_grid_pixel(&mut self.drawing_objects, x, y, value)
    }

    pub fn add_resource(&mut self, unique_id: &str, data: impl Any) {
        self.resources.add(unique_id, data);
    }

    pub fn get_resource<T: Any>(&self, unique_id: &str) -> Option<Ref<'_, T>> {
        self.resources.get_ref(unique_id)
    }

    pub fn query_resources<T: Any>(&self) -> Option<Vec<Ref<'_, T>>> {
        self.resources.query::<T>()
    }

    pub fn borrow_resource(&self) -> &Resource {
        &self.resources
    }

    pub fn borrow_resource_mut(&mut self) -> &mut Resource {
        &mut self.resources
    }

    pub fn init_basic_resources(&mut self) -> Result<(), String> {
        self.resources
            .init_basic_resources(&mut self.drawing_objects)
    }

    pub fn run_window(
        &mut self,
        unique_id: &str,
        message_caller: Rc<RefCell<MessageCaller>>,
    ) -> Result<(), String> {
        let window = self
            .windows
            .get_mut(unique_id)
            .ok_or("no window found".to_string())?;

        window.run(
            &mut self.resources,
            &mut self.drawing_objects,
            message_caller,
            &mut self.benchmark,
        )
    }

    pub fn set_font_active_glyph(
        &mut self,
        unique_id: &str,
        charactere: char,
    ) -> Result<(), String> {
        let window = self.windows.get_mut(unique_id).ok_or("no window found")?;

        window.renderer.font_context.set_active_glyph(charactere)
    }

    pub fn benchmark_print_results(&self) {
        self.benchmark.print_results();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct HealthResource(i32);
    struct EmptyResource();

    #[test]
    fn test_graph_punk_add_resource() {
        let mut graph_punk = GraphPunk::new();

        graph_punk.add_resource("health", HealthResource(42));

        if let Some(value) = graph_punk.get_resource::<HealthResource>("health") {
            assert_eq!(value.0, 42);
        } else {
            assert!(false);
        };
    }

    #[test]
    fn test_graph_punk_query_resource() {
        let mut graph_punk = GraphPunk::new();

        graph_punk.add_resource("health_01", HealthResource(71));
        graph_punk.add_resource("health_02", HealthResource(22));
        graph_punk.add_resource("health_03", HealthResource(99));

        graph_punk.add_resource("empty_01", EmptyResource());
        graph_punk.add_resource("empty_02", EmptyResource());
        graph_punk.add_resource("empty_03", EmptyResource());
        graph_punk.add_resource("empty_04", EmptyResource());
        graph_punk.add_resource("empty_05", EmptyResource());

        if let Some(query) = graph_punk.query_resources::<EmptyResource>() {
            assert_eq!(query.len(), 5);
        } else {
            assert!(false);
        };
    }

    #[test]
    fn test_graph_punk_window_render() -> Result<(), String> {
        let mut graph_punk = GraphPunk::new();

        graph_punk
            .create_window("window", "Window", 100, 100)
            .unwrap();

        graph_punk.window_set_display_size("window", Vec2 { x: 800, y: 600 })?;

        assert!(graph_punk.init_basic_resources().is_ok());

        let message_caller = MessageCaller::default();

        graph_punk.run_window("window", Rc::new(RefCell::new(message_caller)))?;

        Ok(())
    }
}
