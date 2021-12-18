use std::fmt::{Debug};
use crate::{shaders::{UniformValue}, Renderer, Color};

#[derive(Debug, Clone)]
pub struct Material<R: Renderer> {
    pub color: Option<Color>,
    pub texture: Option<R::TextureType>,
    pub shininess: f32
}

impl<R: Renderer> Material<R> {
    pub fn new (color: Color, texture: R::TextureType, shininess: f32) -> Material<R> {
        Material { color: Some(color), texture: Some(texture), shininess }
    }

    pub fn of_color (color: Color, shininess: f32) -> Material<R> {
        Material { color: Some(color), texture: None, shininess }
    }

    pub fn of_texture (texture: R::TextureType, shininess: f32) -> Material<R> {
        Material { color: None, texture: Some(texture), shininess }
    }
}

impl<R: Renderer> UniformValue for Material<R> {
    fn set_to_program<P: crate::shaders::Program>(&self, program: &mut P, key: &str) -> bool where Self: Sized {
        let color = match &self.color {
            None => true,
            Some(color) => {
                let mut name = key.to_string();
                name.push_str(".color");
                color.set_to_program(program, name.as_str())
            }
        };

        let texture = match &self.texture {
            None => true,
            Some(texture) => {
                let mut name = key.to_string();
                name.push_str(".texture");
                texture.set_to_program(program, name.as_str())
            }
        };

        let mut shininess = key.to_string();
        shininess.push_str(".shininess");

        let shininess = self.shininess.set_to_program(program, shininess.as_str());
        color & texture & shininess
    }
}

pub trait Texture: UniformValue + Clone {} 