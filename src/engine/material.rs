use std::fmt::{Debug};
use crate::{shaders::{UniformValue, Uniform}, Renderer, Color};

#[derive(Debug, Clone)]
pub struct Material<R: Renderer> {
    pub color: Option<Color>,
    pub texture: Option<R::TextureType>
}

impl<R: Renderer> Material<R> {
    pub fn new (color: Color, texture: R::TextureType) -> Material<R> {
        Material { color: Some(color), texture: Some(texture) }
    }

    pub fn of_color (color: Color) -> Material<R> {
        Material { color: Some(color), texture: None }
    }

    pub fn of_texture (texture: R::TextureType) -> Material<R> {
        Material { color: None, texture: Some(texture) }
    }
}

impl<R: Renderer> UniformValue for Material<R> {
    fn set_to_program<P: crate::shaders::Program> (&self, program: &P, key: &P::Uniform) -> bool {
        todo!()
    }

    fn set_to_program_by_name<P: crate::shaders::Program>(&self, program: &P, key: &str) -> bool where Self: Sized {
        let color = match &self.color {
            None => true,
            Some(color) => {
                let mut name = key.to_string();
                name.push_str(".color");
                color.set_to_program_by_name(program, name.as_str())
            }
        };

        let texture = match &self.texture {
            None => true,
            Some(texture) => {
                let mut name = key.to_string();
                name.push_str(".texture");
                texture.set_to_program_by_name(program, name.as_str())
            }
        };

        color && texture
    }
}

pub trait Texture: UniformValue + Debug {} 