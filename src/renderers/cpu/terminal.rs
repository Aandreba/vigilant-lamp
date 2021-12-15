use std::sync::{Arc, Mutex};

use crate::Color;
use super::{CanvasCPU, DrawCanvas};

pub struct CanvasTerminal {
    width: u32,
    height: u32
}

impl Default for CanvasTerminal {
    fn default() -> Self {
        Self { width: 50, height: 50}
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
        if color.brightness() >= 0.5 {
            print!("*")
        } else {
            print!("-")
        }

        if (x+1) >= self.get_width() {
            println!()
        }
    }
}

#[test]
fn iter () {
    let mut terminal = Arc::new(Mutex::new(CanvasTerminal::default()));
    terminal.fill_rectangle((0, 0), (10, 10), Color::BLACK);
}