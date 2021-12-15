use crate::{shaders::UniformValue, Color};

pub trait Light: UniformValue {
    fn get_color (&self) -> Color;
    fn set_color (&mut self, color: Color);

    fn get_intensity (&self) -> f32;
    fn set_intensity (&mut self, intensity: f32); 
}