use crate::{color::Color, shaders::{UniformValue, Uniform}, Renderer, OptionFlatMap};

#[derive(Debug)]
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
    fn set_to_program<P: crate::shaders::Program> (self, program: &P, key: &P::Uniform) -> bool {
        let color = match self.color {
            None => true,
            Some(color) => { 
                key.get_child("color", program)
                    .map(|k| color.set_to_program(program, k))
                    .unwrap_or(false)
            }
        };

        let texture = match self.texture {
            None => true,
            Some(texture) => { 
                key.get_child("texture", program)
                    .map(|k| texture.set_to_program(program, k))
                    .unwrap_or(false)
            }
        };

        color && texture
    }
}

pub trait Texture: UniformValue {} 