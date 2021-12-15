use crate::{vector::EucVecf3, shaders::{UniformValue, subkey}, Color};
use super::{Light, AmbientLight};

pub struct PointLight {
    pub position: EucVecf3,
    ambient: AmbientLight
}

impl PointLight {
    pub fn new (color: Color, intensity: f32) -> AmbientLight {
        AmbientLight { color, intensity }
    }
}

impl UniformValue for PointLight {
    fn set_to_program<P: crate::shaders::Program>(&self, program: &mut P, key: &str) -> bool where Self: Sized {
        if !self.position.set_to_program(program, subkey(key, "position").as_str()) {
            return false
        }

        self.ambient.set_to_program(program, subkey(key, "light").as_str())
    }
}

impl Light for PointLight {
    fn get_color (&self) -> crate::Color {
        self.ambient.color
    }

    fn set_color (&mut self, color: crate::Color) {
        self.ambient.color = color
    }

    fn get_intensity (&self) -> f32 {
        self.ambient.intensity
    }

    fn set_intensity (&mut self, intensity: f32) {
        self.ambient.intensity = intensity
    }
}