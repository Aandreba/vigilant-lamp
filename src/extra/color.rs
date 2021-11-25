pub struct Color (u32);

// CONSTANTS
impl Color {
    pub const WHITE : Color = Color(u32::MAX);
    pub const BLACK : Color = Color(255);
    pub const TRANSPARENT : Color = Color(0);

    pub const RED : Color = Color(0xFF0000FF);
    pub const GREEN : Color = Color(0x00FF00FF);
    pub const BLUE : Color = Color(0x0000FFFF);

    pub const PINK : Color = Color(0xFFAFAFFF);
    pub const ORANGE : Color = Color(0xFFC800FF);
    pub const YELLOW : Color = Color(0xFFFF00FF);
    pub const CYAN : Color = Color(0x00FFFFFF);
}

// INITIALIZERS
impl Color {
    pub fn from_rgba (r: u8, g: u8, b: u8, a: u8) -> Color {
        let alpha = a as u32;
        let blue = (b as u32) << 8;
        let green = (g as u32) << 16;
        let red = (r as u32) << 24;

        Color(red | green | blue | alpha)
    }

    pub fn from_rgb (r: u8, g: u8, b: u8) -> Color {
        let blue = (b as u32) << 8;
        let green = (g as u32) << 16;
        let red = (r as u32) << 24;

        Color(red | green | blue | 255)
    }

    pub fn from_hex (value: &str) -> Option<Color> {
        let hex : &str;

        if value.chars().next().unwrap() == '#' {
            hex = &value[1..]
        } else {
            hex = value;
        }

        if hex.len() == 6 {
            let parse = u32::from_str_radix(hex, 16);
            match parse {
                Ok(x) => return Some(Color(x << 8)),
                Err(_) => return None
            }
            
        } else if hex.len() == 8 {
            let parse = u32::from_str_radix(hex, 16);
            match parse {
                Ok(x) => return Some(Color(x)),
                Err(_) => return None
            }
        }

        None
    }
}

// METHODS
impl Color {
    pub fn alpha (&self) -> u8 {
        (self.0 & 255) as u8
    }

    pub fn alpha_f32 (&self) -> f32 {
        (self.alpha() as f32) / 255.
    }

    // RGB
    pub fn red (&self) -> u8 {
        ((self.0 >> 24) & 255) as u8
    }

    pub fn green (&self) -> u8 {
        ((self.0 >> 16) & 255) as u8
    }
    
    pub fn blue (&self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    pub fn red_f32 (&self) -> f32 {
        (self.red() as f32) / 255.
    }

    pub fn green_f32 (&self) -> f32 {
        (self.green() as f32) / 255.
    }

    pub fn blue_f32 (&self) -> f32 {
        (self.blue() as f32) / 255.
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

    // COMPONENTS
    pub fn rgba_components (&self) -> [u8;4] {
        [self.red(), self.green(), self.blue(), self.alpha()]
    }

    pub fn argb_components (&self) -> [u8;4] {
        [self.alpha(), self.red(), self.green(), self.blue()]
    }

    pub fn rgb_components (&self) -> [u8;3] {
        [self.red(), self.green(), self.blue()]
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