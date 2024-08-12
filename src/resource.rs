use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    time::Instant,
};

use crate::{
    drawing::{rectangle::Rectangle, text::Text},
    maths::vec::{Vec3, Vec4},
    renderer::draw::Draw,
    shader::{program::ShaderProgram, Shader, ShaderType},
};

pub mod gl_resource;

pub struct Resource {
    data: HashMap<TypeId, HashMap<String, ResourceInfo>>,
}

pub struct ResourceInfo {
    instant: Instant,
    value: Rc<RefCell<dyn Any>>,
}

impl Resource {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, unique_id: &str, value: impl Any) {
        let type_id = value.type_id();
        let cell_value = Rc::new(RefCell::new(value));

        let hashmap = {
            if let Some(h) = self.data.get_mut(&type_id) {
                h
            } else {
                self.data.insert(type_id, HashMap::new());

                self.data.get_mut(&type_id).unwrap()
            }
        };

        hashmap.insert(
            unique_id.to_string(),
            ResourceInfo {
                instant: Instant::now(),
                value: cell_value,
            },
        );
    }

    pub fn get_ref<T: Any>(&self, unique_id: &str) -> Option<Ref<'_, T>> {
        // On retourne un T et non pas un &T car il est dans stocké dans une structure 'Ref', qui
        // est l'équivalent d'un &, donc il n'est pas nécessaire d'utiliser &T.
        let type_id = TypeId::of::<T>();

        // `and_then` retourne None si 'Option' vaut None.
        // Sinon exécute la fonction anonyme avec comme paramètre la valeur de l'Option.
        self.data.get(&type_id).and_then(|hashmap| {
            hashmap.get(unique_id).and_then(|resource_info| {
                // Essaye d'emprunter une référence et retourne None si une erreur survint.
                let ref_any = resource_info.value.try_borrow().ok()?;

                // 'Ref::map' permet de convertir une 'Ref' dans un autre format.
                let ref_t = Ref::map(ref_any, |any| {
                    any.downcast_ref::<T>()
                        .expect("TypeId matched but downcast failed")
                });

                Some(ref_t)
            })
        })
    }

    pub fn query<T: Any>(&self) -> Option<Vec<Ref<'_, T>>> {
        let type_id = TypeId::of::<T>();

        self.data.get(&type_id).and_then(|hashmap| {
            let mut v = Vec::new();

            // Itère à travers la 'HashMap'.
            for (_, resource_info) in hashmap {
                let ref_any = resource_info.value.try_borrow().ok()?;

                let ref_t = Ref::map(ref_any, |any| {
                    any.downcast_ref::<T>()
                        .expect("TypeId matched but downcast failed")
                });

                v.push((resource_info.instant, ref_t));
            }

            v.sort_by(|a, b| a.0.cmp(&b.0));

            let v = v.into_iter().map(|element| element.1).collect();

            Some(v)
        })
    }

    pub fn query_mut<T: Any>(&mut self) -> Option<Vec<RefMut<'_, T>>> {
        let type_id = TypeId::of::<T>();

        self.data.get_mut(&type_id).and_then(|hashmap| {
            let mut v = Vec::new();

            // Itère à travers la 'HashMap'.
            for (_, resource_info) in hashmap {
                let ref_any = resource_info.value.try_borrow_mut().ok()?;

                let ref_t = RefMut::map(ref_any, |any| {
                    any.downcast_mut::<T>()
                        .expect("TypeId matched but downcast failed")
                });

                v.push((resource_info.instant, ref_t));
            }

            v.sort_by(|a, b| a.0.cmp(&b.0));

            let v = v.into_iter().map(|element| element.1).collect();

            Some(v)
        })
    }

    pub fn init_basic_resources(
        &mut self,
        drawing_objects: &mut Vec<Box<dyn Draw>>,
    ) -> Result<(), String> {
        let basic_2d_vertex_shader = include_str!("../Builtin/Shaders/basic_2D_vertex_shader.glsl");
        let basic_2d_fragment_shader =
            include_str!("../Builtin/Shaders/basic_2D_fragment_shader.glsl");

        let basic_text_vertex_shader =
            include_str!("../Builtin/Shaders/basic_text_vertex_shader.glsl");
        let basic_text_fragment_shader =
            include_str!("../Builtin/Shaders/basic_text_fragment_shader.glsl");

        let mut vert_shader = Shader::new(
            ShaderType::Vertex,
            "basic_2D_vertex_shader".to_string(),
            basic_2d_vertex_shader.to_string(),
        );

        vert_shader.create()?;
        vert_shader.source()?;
        vert_shader.compile()?;

        let mut frag_shader = Shader::new(
            ShaderType::Fragment,
            "basic_2D_fragment_shader".to_string(),
            basic_2d_fragment_shader.to_string(),
        );

        frag_shader.create()?;
        frag_shader.source()?;
        frag_shader.compile()?;

        let program = ShaderProgram::build(&vert_shader, &frag_shader)?;

        program.link()?;

        let mut text_vert_shader = Shader::new(
            ShaderType::Vertex,
            "basic_text_vertex_shader".to_string(),
            basic_text_vertex_shader.to_string(),
        );

        text_vert_shader.create()?;
        text_vert_shader.source()?;
        text_vert_shader.compile()?;

        let mut text_frag_shader = Shader::new(
            ShaderType::Fragment,
            "basic_text_fragment_shader".to_string(),
            basic_text_fragment_shader.to_string(),
        );

        text_frag_shader.create()?;
        text_frag_shader.source()?;
        text_frag_shader.compile()?;

        let text_program = ShaderProgram::build(&text_vert_shader, &text_frag_shader)?;

        text_program.link()?;

        let rect = Rectangle::build(
            program,
            Vec4 {
                x: 1.0_f32,
                y: 1.0_f32,
                z: 1.0_f32,
                w: 1.0_f32,
            },
            Vec3 {
                x: 0.0_f32,
                y: 0.0_f32,
                z: 0.0_f32,
            },
            Vec3 {
                x: 1.0_f32,
                y: 1.0_f32,
                z: 1.0_f32,
            },
        )?;

        for y in 0..32 {
            for x in 0..64 {
                let mut rect_clone = rect.clone();
                rect_clone.set_position(Vec3 {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0_f32,
                });
                rect_clone.set_visible(false);

                drawing_objects.push(Box::new(rect_clone));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestResource(i32);
    struct QueryResource();

    #[test]
    fn test_add_resource() {
        let mut resource = Resource::new();

        resource.add("test", TestResource(22));

        if let Some(value) = resource.get_ref::<TestResource>("test") {
            assert_eq!(value.0, 22);
        } else {
            assert!(false);
        };
    }

    #[test]
    fn test_query_resources() {
        let mut resource = Resource::new();

        resource.add("test_01", TestResource(11));
        resource.add("test_02", TestResource(22));

        resource.add("query_01", QueryResource());
        resource.add("query_02", QueryResource());
        resource.add("query_03", QueryResource());

        if let Some(query) = resource.query::<QueryResource>() {
            assert_eq!(query.len(), 3);
        } else {
            assert!(false);
        };
    }
}
