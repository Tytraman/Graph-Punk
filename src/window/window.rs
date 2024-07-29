use std::{cell::RefCell, rc::Rc};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    sys::SDL_GL_SetSwapInterval,
    EventPump, Sdl,
};

use crate::{
    gl_exec,
    message::MessageCaller,
    renderer::Renderer,
    resource::{renderer_resource::DrawingResource, Resource},
    types::{UserData, RGB},
};
use crate::{maths::mat::Mat4, renderer::check_errors};
use crate::{maths::vec::Vec2, renderer::clear_errors};

use super::user_input::{KeyStatus, Keys};

pub struct Window<'a> {
    sdl: Sdl,
    window: sdl2::video::Window,
    event_pump: EventPump,
    renderer: Renderer,
    keys: Keys,
    background_color: RGB,
    update_callback: Box<dyn FnMut(&Keys, &mut UserData) + 'a>,
    user_data: UserData,
}

impl<'a> Window<'a> {
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
        display_size: Vec2<i32>,
    ) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        // Défini les options globales d'OpenGL, nécessaire avant de se servir de la moindre
        // fonction OpenGL.
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Crée la fenêtre.
        let window = match video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .position_centered()
            .build()
        {
            Ok(t) => t,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Permet de récupérer les évènements liés à la fenêtre, comme les entrées utilisateur.
        let event_pump = sdl.event_pump()?;

        // Crée le contexte OpenGL nécessaire pour la fenêtre afin de dessiner dessus.
        let gl_context = window.gl_create_context()?;

        // Charge toutes les fonctions OpenGL grâce à une recherche customisée.
        let _ = gl::load_with(|proc_name| {
            video_subsystem.gl_get_proc_address(proc_name) as *const std::os::raw::c_void
        });

        let renderer = Renderer::new(gl_context, display_size);
        if let Err(err) = renderer.set_viewport_size(width as i32, height as i32) {
            return Err(err);
        }

        unsafe { SDL_GL_SetSwapInterval(0) };

        Ok(Window {
            sdl,
            window,
            event_pump,
            renderer,
            keys: Keys::new(),
            background_color: RGB::new(0, 0, 0),
            update_callback: Box::new(|_, _| {}),
            user_data: UserData { data: None },
        })
    }

    pub fn run(
        &mut self,
        resource: &mut Resource,
        message_caller: Rc<RefCell<MessageCaller>>,
    ) -> Result<(), String> {
        // Boucle infinie de la fenêtre.
        'running: loop {
            self.keys.update_last_key_states();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode,
                        repeat: false,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Num0 => self.keys.set_key_state("0", KeyStatus::Pressed),
                                Keycode::Num1 => self.keys.set_key_state("1", KeyStatus::Pressed),
                                Keycode::Num2 => self.keys.set_key_state("2", KeyStatus::Pressed),
                                Keycode::Num3 => self.keys.set_key_state("3", KeyStatus::Pressed),
                                Keycode::Num4 => self.keys.set_key_state("4", KeyStatus::Pressed),
                                Keycode::Num5 => self.keys.set_key_state("5", KeyStatus::Pressed),
                                Keycode::Num6 => self.keys.set_key_state("6", KeyStatus::Pressed),
                                Keycode::Num7 => self.keys.set_key_state("7", KeyStatus::Pressed),
                                Keycode::Num8 => self.keys.set_key_state("8", KeyStatus::Pressed),
                                Keycode::Num9 => self.keys.set_key_state("9", KeyStatus::Pressed),
                                Keycode::A => self.keys.set_key_state("a", KeyStatus::Pressed),
                                Keycode::B => self.keys.set_key_state("b", KeyStatus::Pressed),
                                Keycode::C => self.keys.set_key_state("c", KeyStatus::Pressed),
                                Keycode::D => self.keys.set_key_state("d", KeyStatus::Pressed),
                                Keycode::E => self.keys.set_key_state("e", KeyStatus::Pressed),
                                Keycode::F => self.keys.set_key_state("f", KeyStatus::Pressed),
                                Keycode::G => self.keys.set_key_state("g", KeyStatus::Pressed),
                                Keycode::H => self.keys.set_key_state("h", KeyStatus::Pressed),
                                Keycode::I => self.keys.set_key_state("i", KeyStatus::Pressed),
                                Keycode::J => self.keys.set_key_state("j", KeyStatus::Pressed),
                                Keycode::K => self.keys.set_key_state("k", KeyStatus::Pressed),
                                Keycode::L => self.keys.set_key_state("l", KeyStatus::Pressed),
                                Keycode::M => self.keys.set_key_state("m", KeyStatus::Pressed),
                                Keycode::N => self.keys.set_key_state("n", KeyStatus::Pressed),
                                Keycode::O => self.keys.set_key_state("o", KeyStatus::Pressed),
                                Keycode::P => self.keys.set_key_state("p", KeyStatus::Pressed),
                                Keycode::Q => self.keys.set_key_state("q", KeyStatus::Pressed),
                                Keycode::R => self.keys.set_key_state("r", KeyStatus::Pressed),
                                Keycode::S => self.keys.set_key_state("s", KeyStatus::Pressed),
                                Keycode::T => self.keys.set_key_state("t", KeyStatus::Pressed),
                                Keycode::U => self.keys.set_key_state("u", KeyStatus::Pressed),
                                Keycode::V => self.keys.set_key_state("v", KeyStatus::Pressed),
                                Keycode::W => self.keys.set_key_state("w", KeyStatus::Pressed),
                                Keycode::X => self.keys.set_key_state("x", KeyStatus::Pressed),
                                Keycode::Y => self.keys.set_key_state("y", KeyStatus::Pressed),
                                Keycode::Z => self.keys.set_key_state("z", KeyStatus::Pressed),
                                Keycode::Space => self.keys.set_key_state(" ", KeyStatus::Pressed),
                                _ => (),
                            }
                        }
                    }
                    Event::KeyUp {
                        keycode,
                        repeat: false,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Num0 => self.keys.set_key_state("0", KeyStatus::Released),
                                Keycode::Num1 => self.keys.set_key_state("1", KeyStatus::Released),
                                Keycode::Num2 => self.keys.set_key_state("2", KeyStatus::Released),
                                Keycode::Num3 => self.keys.set_key_state("3", KeyStatus::Released),
                                Keycode::Num4 => self.keys.set_key_state("4", KeyStatus::Released),
                                Keycode::Num5 => self.keys.set_key_state("5", KeyStatus::Released),
                                Keycode::Num6 => self.keys.set_key_state("6", KeyStatus::Released),
                                Keycode::Num7 => self.keys.set_key_state("7", KeyStatus::Released),
                                Keycode::Num8 => self.keys.set_key_state("8", KeyStatus::Released),
                                Keycode::Num9 => self.keys.set_key_state("9", KeyStatus::Released),
                                Keycode::A => self.keys.set_key_state("a", KeyStatus::Released),
                                Keycode::B => self.keys.set_key_state("b", KeyStatus::Released),
                                Keycode::C => self.keys.set_key_state("c", KeyStatus::Released),
                                Keycode::D => self.keys.set_key_state("d", KeyStatus::Released),
                                Keycode::E => self.keys.set_key_state("e", KeyStatus::Released),
                                Keycode::F => self.keys.set_key_state("f", KeyStatus::Released),
                                Keycode::G => self.keys.set_key_state("g", KeyStatus::Released),
                                Keycode::H => self.keys.set_key_state("h", KeyStatus::Released),
                                Keycode::I => self.keys.set_key_state("i", KeyStatus::Released),
                                Keycode::J => self.keys.set_key_state("j", KeyStatus::Released),
                                Keycode::K => self.keys.set_key_state("k", KeyStatus::Released),
                                Keycode::L => self.keys.set_key_state("l", KeyStatus::Released),
                                Keycode::M => self.keys.set_key_state("m", KeyStatus::Released),
                                Keycode::N => self.keys.set_key_state("n", KeyStatus::Released),
                                Keycode::O => self.keys.set_key_state("o", KeyStatus::Released),
                                Keycode::P => self.keys.set_key_state("p", KeyStatus::Released),
                                Keycode::Q => self.keys.set_key_state("q", KeyStatus::Released),
                                Keycode::R => self.keys.set_key_state("r", KeyStatus::Released),
                                Keycode::S => self.keys.set_key_state("s", KeyStatus::Released),
                                Keycode::T => self.keys.set_key_state("t", KeyStatus::Released),
                                Keycode::U => self.keys.set_key_state("u", KeyStatus::Released),
                                Keycode::V => self.keys.set_key_state("v", KeyStatus::Released),
                                Keycode::W => self.keys.set_key_state("w", KeyStatus::Released),
                                Keycode::X => self.keys.set_key_state("x", KeyStatus::Released),
                                Keycode::Y => self.keys.set_key_state("y", KeyStatus::Released),
                                Keycode::Z => self.keys.set_key_state("z", KeyStatus::Released),
                                Keycode::Space => self.keys.set_key_state(" ", KeyStatus::Released),
                                _ => (),
                            }
                        }
                    }
                    // Détecte lorsque la fenêtre est redimensionnée.
                    Event::Window {
                        win_event: WindowEvent::Resized(width, height),
                        ..
                    } => {
                        if let Err(err) = self.renderer.set_viewport_size(width, height) {
                            eprintln!("{err}");
                        }

                        let display_size = self.renderer.get_display_size();
                        let display_aspect_ratio = display_size.x as f32 / display_size.y as f32;

                        // Le rapport d'aspect permet d'agrandir / réduire le rendu afin de
                        // remplir l'espace disponible.
                        let aspect_ratio = width as f32 / height as f32;
                        self.renderer.aspect_ratio = aspect_ratio;

                        // Si la largeur est plus grande que la hauteur alors il faut scale sur
                        // la largeur.
                        if aspect_ratio >= display_aspect_ratio {
                            // Viewport plus large, utilise toute la hauteur.
                            self.renderer.left = -aspect_ratio / display_aspect_ratio
                                * display_size.x as f32
                                / 2.0_f32;
                            self.renderer.bottom = display_size.y as f32 / 2.0_f32;

                            self.renderer.projection = Mat4::ortho(
                                self.renderer.left as i32 - 1,
                                -self.renderer.left as i32,
                                self.renderer.bottom as i32,
                                -self.renderer.bottom as i32 - 1,
                                -100.0_f32,
                                100.0_f32,
                            );
                        // Sinon il faut scale sur la hauteur.
                        } else {
                            // Viewport plus haut, utilise toute la largeur.
                            self.renderer.left = -display_size.x as f32 / 2.0_f32;
                            self.renderer.bottom = display_aspect_ratio / aspect_ratio
                                * display_size.y as f32
                                / 2.0_f32;

                            self.renderer.projection = Mat4::ortho(
                                self.renderer.left as i32 - 1,
                                -self.renderer.left as i32,
                                self.renderer.bottom as i32,
                                -self.renderer.bottom as i32,
                                -100.0_f32,
                                100.0_f32,
                            );
                        }
                    }
                    _ => {}
                }
            }

            // Appelle la fonction de callback pour mettre à jour l'état du moteur et du programme.
            (self.update_callback)(&self.keys, &mut self.user_data);

            message_caller
                .borrow_mut()
                .execute(&mut self.renderer, resource);

            // Défini la couleur qu'OpenGL va utiliser pour nettoyer l'écran.
            if let Err(err) = gl_exec!(|| gl::ClearColor(
                self.background_color.r as f32 / 255.0 as f32,
                self.background_color.g as f32 / 255.0 as f32,
                self.background_color.b as f32 / 255.0 as f32,
                1.0 as f32,
            )) {
                return Err(err);
            }

            // Nettoie l'écran.
            if let Err(err) = gl_exec!(|| gl::Clear(gl::COLOR_BUFFER_BIT)) {
                return Err(err);
            }

            // Dessine tous les objets.
            if let Some(query) = resource.query::<DrawingResource>() {
                for drawing_object in query.iter() {
                    if drawing_object.0.is_visible() {
                        if let Err(err) = drawing_object
                            .0
                            .draw(&self.renderer, &self.renderer.projection)
                        {
                            eprintln!("{err}");

                            continue;
                        }
                    }
                }
                // Met à jour le contenu dessiné sur la fenêtre.
                self.window.gl_swap_window();
            }
        }

        Ok(())
    }

    pub fn set_display_size(&mut self, size: Vec2<i32>) {
        self.renderer.set_display_size(size);
    }

    pub fn get_width(&self) -> u32 {
        self.window.size().0
    }

    pub fn get_height(&self) -> u32 {
        self.window.size().1
    }

    pub fn borrow_sdl(&self) -> &Sdl {
        &self.sdl
    }

    pub fn borrow_renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn borrow_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn set_update_callback(
        &mut self,
        c: impl FnMut(&Keys, &mut UserData) + 'a,
        user_data: UserData,
    ) {
        self.update_callback = Box::new(c);
        self.user_data = user_data;
    }
}
