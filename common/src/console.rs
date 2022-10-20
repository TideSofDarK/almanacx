use std::cell::RefMut;

const BG_COLOR: (u8, u8, u8, f32) = (100, 105, 80, 0.8);
pub struct Console {
    width: u32,
    height: u32,
    open: bool,
    y_offset: f32,
    moving: bool,
}

impl Console {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height / 3,
            open: false,
            y_offset: 0.0,
            moving: false,
        }
    }

    pub fn handle_input(&mut self) {}

    pub fn update(&mut self, dt: f32) {
        if self.moving {
            let sign = match self.open {
                true => 1.0,
                false => -1.0,
            };
            self.y_offset = (self.y_offset as f32 + (500.0 * dt * sign)).clamp(0.0, 255.0);

            if self.y_offset >= self.height as f32 || self.y_offset <= 0.0 {
                self.moving = false;
            }
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        if self.y_offset <= 0.0 {
            return;
        }

        for y in 0..self.y_offset as usize {
            for x in 0..self.width {
                let index = (y as usize * self.width as usize + x as usize) * 4;

                frame[index] = lerp_u8(frame[index], BG_COLOR.0, BG_COLOR.3);
                frame[index + 1] = lerp_u8(frame[index + 1], BG_COLOR.1, BG_COLOR.3);
                frame[index + 2] = lerp_u8(frame[index + 2], BG_COLOR.2, BG_COLOR.3);
                frame[index + 3] = u8::MAX;
            }
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.moving = true;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).clamp(0.0, 255.0) as u8
}
