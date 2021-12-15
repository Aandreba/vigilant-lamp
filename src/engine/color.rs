use std::fmt::Debug;
use crate::{shaders::UniformValue, vector::EucVecf4};

/// Representation of a color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

// CONSTANTS
impl Color {
    pub const WHITE : Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK : Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const TRANSPARENT : Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub const RED : Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN : Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE : Color = Color { r: 0, g: 0, b: 255, a: 255 };

    pub const PINK : Color = Color { r: 255, g: 175, b: 175, a: 255 };
    pub const ORANGE : Color = Color { r: 255, g: 200, b: 0, a: 255 };
    pub const YELLOW : Color = Color { r: 255, g: 255, b: 0, a: 255 };
    pub const CYAN : Color = Color { r: 0, g: 255, b: 255, a: 255 };
}

// INITIALIZERS
impl Color {
    pub fn new (r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r, g, b, a}
    }

    pub fn from_rgb (r: u8, g: u8, b: u8) -> Color {
        Color::new(r, g, b, 255)
    }
}

// METHODS
impl Color {
    pub fn alpha_f32 (&self) -> f32 {
        (self.a as f32) / 255.
    }

    // RGB
    pub fn red_f32 (&self) -> f32 {
        (self.r as f32) / 255.
    }

    pub fn green_f32 (&self) -> f32 {
        (self.g as f32) / 255.
    }

    pub fn blue_f32 (&self) -> f32 {
        (self.b as f32) / 255.
    }

    // HSV
    pub fn hue (&self) -> f32 {
        let r = self.red_f32();
        let g = self.green_f32();
        let b = self.blue_f32();

        let min = r.min(g.min(b));
        let max = r.max(g.max(b));
        let delta = max - min;

        if delta == 0. {
            0.
        } else if max == r {
            (((g - b) / delta) % 6.) / 6.
        } else if max == g {
            (((b - r) / delta) + 2.) / 6.
        } else {
            (((r - g) / delta) + 4.) / 6.
        }
    }

    pub fn saturation (&self) -> f32 {
        let r = self.red_f32();
        let g = self.green_f32();
        let b = self.blue_f32();

        let max = r.max(g.max(b));
        if max == 0. {
            return 0.;
        }

        let min = r.min(g.min(b));
        let delta = max - min;

        delta / max
    }

    pub fn brightness (&self) -> f32 {
        let r = self.red_f32();
        let g = self.green_f32();
        let b = self.blue_f32();

        r.max(g.max(b))
    } 

    pub fn intensity (&self) -> f32 {
        ((self.r as f32) + (self.g as f32) + (self.b as f32)) / 765.
    } 

    // COMPONENTS
    pub fn rgba_components (&self) -> [u8;4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn argb_components (&self) -> [u8;4] {
        [self.a, self.r, self.g, self.b]
    }

    pub fn rgb_components (&self) -> [u8;3] {
        [self.r, self.g, self.b]
    }

    pub fn rgba_components_f32 (&self) -> [f32;4] {
        [self.red_f32(), self.green_f32(), self.blue_f32(), self.alpha_f32()]
    }

    pub fn argb_components_f32 (&self) -> [f32;4] {
        [self.alpha_f32(), self.red_f32(), self.green_f32(), self.blue_f32()]
    }

    pub fn rgb_components_f32 (&self) -> [f32;3] {
        [self.red_f32(), self.green_f32(), self.blue_f32()]
    }

    pub fn hsv_components (&self) -> [f32;3] {
        let r = self.red_f32();
        let g = self.green_f32();
        let b = self.blue_f32();

        let min = r.min(g.min(b));
        let max = r.max(g.max(b));
        let delta = max - min;

        let hue = 
            if delta == 0. {
                0.
            } else if max == r {
                (((g - b) / delta) % 6.) / 6.
            } else if max == g {
                (((b - r) / delta) + 2.) / 6.
            } else {
                (((r - g) / delta) + 4.) / 6.
            };

        let saturation = if max == 0. { 0. } else { delta / max };
        [hue, saturation, max]
    }
}

impl UniformValue for Color {
    fn set_to_program<P: crate::shaders::Program> (&self, program: &mut P, key: &str) -> bool {
        let vec = EucVecf4::new(self.red_f32(), self.green_f32(), self.blue_f32(), self.alpha_f32());
        vec.set_to_program(program, key)
    }
}