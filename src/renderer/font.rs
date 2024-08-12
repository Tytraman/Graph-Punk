use freetype::{
    freetype::{
        FT_Done_Face, FT_Done_FreeType, FT_Face, FT_Init_FreeType, FT_Int32, FT_Library,
        FT_Load_Char, FT_New_Memory_Face, FT_Set_Pixel_Sizes, FT_ULong, FT_LOAD_RENDER,
    },
    succeeded,
};
use gl::types::{GLint, GLsizei, GLuint};
use std::{collections::HashMap, os, ptr};

use crate::{gl_exec, maths::vec::Vec2, punk_error};

pub struct FontContext {
    lib: FT_Library,
    face_font: FT_Face,
    pub(crate) characteres: HashMap<char, Charactere>,
}

pub struct Charactere {
    pub(crate) texture_id: GLuint,
    pub(crate) size: Vec2<u32>,
    pub(crate) bearing: Vec2<i32>,
    pub(crate) advance_x: i64,
}

impl FontContext {
    pub fn build() -> Result<Self, String> {
        let font = include_bytes!("../../Builtin/Fonts/DejaVuSans.ttf");

        let mut lib: FT_Library = ptr::null_mut();

        let mut result = unsafe {
            // Initialise la librarie FreeType.
            FT_Init_FreeType(&mut lib)
        };

        if !succeeded(result) {
            return Err("cannot initialize FreeType library".to_string());
        }

        let mut face: FT_Face = ptr::null_mut();

        result = unsafe {
            // Charge une police d'écriture depuis la mémoire.
            FT_New_Memory_Face(lib, font.as_ptr(), font.len() as i64, 0, &mut face)
        };

        if !succeeded(result) {
            unsafe {
                FT_Done_FreeType(lib);
            }

            return Err("cannot load default font face".to_string());
        }

        unsafe {
            // Défini la taille de la police d'écriture.
            FT_Set_Pixel_Sizes(face, 0, 48);
        }

        // Génère et stock toutes les textures des caractères afin de ne pas avoir à le faire à
        // chaque frame.

        // Désactive l'alignement des octets dans OpenGL.
        if let Err(err) = gl_exec!(|| gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1)) {
            unsafe {
                FT_Done_Face(face);
                FT_Done_FreeType(lib);
            }

            return Err(err);
        }

        let mut characteres = HashMap::new();

        for c in 0_u32..128 {
            // Charge le glyph du caractère.
            result = unsafe { FT_Load_Char(face, c as FT_ULong, FT_LOAD_RENDER as FT_Int32) };

            if !succeeded(result) {
                punk_error!("cannot load glyph");
                continue;
            }

            // Génère la texture.
            let mut texture_id = 0;
            if let Err(err) = gl_exec!(|| gl::GenTextures(1, &mut texture_id)) {
                punk_error!("{err}");
                continue;
            }

            // Rend la texture nouvellement générée active.
            if let Err(err) = gl_exec!(|| gl::BindTexture(gl::TEXTURE_2D, texture_id)) {
                punk_error!("{err}");
                let _ = gl_exec!(|| gl::DeleteTextures(1, &texture_id));
                continue;
            }

            let width = unsafe { (*(*face).glyph).bitmap.width };
            let height = unsafe { (*(*face).glyph).bitmap.rows };

            // Transfère les données du caractères dans la texture actuellement active.
            if let Err(err) = gl_exec!(|| gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                (*(*face).glyph).bitmap.buffer as *const os::raw::c_void,
            )) {
                punk_error!("{err}");
                let _ = gl_exec!(|| gl::DeleteTextures(1, &texture_id));
                continue;
            }

            // Défini les options de la texture.
            let _ = gl_exec!(|| gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as GLint
            ));
            let _ = gl_exec!(|| gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as GLint
            ));
            let _ = gl_exec!(|| gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as GLint
            ));
            let _ = gl_exec!(|| gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as GLint
            ));

            let charactere = Charactere {
                texture_id,
                size: Vec2 {
                    x: width,
                    y: height,
                },
                bearing: Vec2 {
                    x: unsafe { (*(*face).glyph).bitmap_left },
                    y: unsafe { (*(*face).glyph).bitmap_top },
                },
                advance_x: unsafe { (*(*face).glyph).advance.x },
            };

            characteres.insert(char::from_u32(c).unwrap(), charactere);
        }

        // Réactive l'alignement des octets dans OpenGL.
        if let Err(err) = gl_exec!(|| gl::PixelStorei(gl::PACK_ALIGNMENT, 1)) {
            unsafe {
                FT_Done_Face(face);
                FT_Done_FreeType(lib);
            }

            return Err(err);
        }

        Ok(Self {
            lib,
            face_font: face,
            characteres,
        })
    }

    pub fn set_active_glyph(&mut self, charactere: char) -> Result<(), String> {
        let result = unsafe {
            FT_Load_Char(
                self.face_font,
                charactere as FT_ULong,
                FT_LOAD_RENDER as FT_Int32,
            )
        };

        if !succeeded(result) {
            return Err("cannot set active glyph".to_string());
        }

        Ok(())
    }

    pub fn get_active_glyph_width(&self) -> u32 {
        unsafe { (*(*self.face_font).glyph).bitmap.width }
    }

    pub fn get_active_glyph_height(&self) -> u32 {
        unsafe { (*(*self.face_font).glyph).bitmap.rows }
    }

    pub fn get_active_glyph_bearing_x(&self) -> i32 {
        unsafe { (*(*self.face_font).glyph).bitmap_left }
    }

    pub fn get_active_glyph_bearing_y(&self) -> i32 {
        unsafe { (*(*self.face_font).glyph).bitmap_top }
    }

    pub fn get_active_glyph_advance_x(&self) -> i64 {
        unsafe { (*(*self.face_font).glyph).advance.x }
    }

    pub fn get_active_glyph_advance_y(&self) -> i64 {
        unsafe { (*(*self.face_font).glyph).advance.y }
    }
}
