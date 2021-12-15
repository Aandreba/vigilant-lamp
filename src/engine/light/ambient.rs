use crate::{Color, shaders::{UniformValue, subkey}};

use super::Light;

pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32
}

impl AmbientLight {
    pub fn new (color: Color, intensity: f32) -> AmbientLight {
        AmbientLight { color, intensity }
    }
}

impl UniformValue for AmbientLight {
    fn set_to_program<P: crate::shaders::Program>(&self, program: &mut P, key: &str) -> bool where Self: Sized {
        if !self.color.set_to_program(program, subkey(key, "color").as_str()) {
            return false
        }

        if !self.intensity.set_to_program(program, subkey(key, "intensity").as_str()) {
            return false
        }

        true
    }
}

impl Light for AmbientLight {
    fn get_color (&self) -> crate::Color {
        self.color
    }

    fn set_color (&mut self, color: crate::Color) {
        self.color = color
    }

    fn get_intensity (&self) -> f32 {
        self.intensity
    }

    fn set_intensity (&mut self, intensity: f32) {
        self.intensity = intensity
    }
}