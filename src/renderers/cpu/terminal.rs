use std::{collections::{HashMap}, str::Chars};
use crate::Color;
use super::{CanvasCPU};

const GRAYSCALE : &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct CanvasTerminal {
    width: u32,
    height: u32,
    map: HashMap<(u32, u32), Color>
}

impl Default for CanvasTerminal {
    fn default() -> Self {
        Self::new(50, 50)
    }
} 

impl CanvasTerminal {
    pub fn new (width: u32, height: u32) -> Self {
        Self { width, height, map: HashMap::new() }
    }

    pub fn print (&self) {
        let len = (GRAYSCALE.len() - 1) as f32;
        for j in 0..self.height {
            for i in 0..self.width {
                let color = self.map.get(&(i, j)).unwrap_or(&Color::BLACK);
                let index = (color.intensity() * len).round() as usize;
                print!("{}", GRAYSCALE.chars().nth(index).unwrap());
            } 

            println!();
        }
    }
}

impl CanvasCPU for CanvasTerminal {
    fn get_width (&self) -> u32 {
        self.width
    }

    fn get_height (&self) -> u32 {
        self.height
    }

    fn set_pixel (&mut self, x: u32, y: u32, color: crate::Color) {
        self.map.insert((x, y), color);
    }
}