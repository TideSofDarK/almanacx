use crate::{buffer2d::B2DS, platform::input::{Input, InputCode}, utils::color_from_tuple};

const CONSOLE_COLOR: u16 = color_from_tuple((2,2,2));

pub struct Console {
    width: i32,
    open: bool,
    x_offset: f32,
    moving: bool,
}

impl Console {
    pub fn new(width: i32) -> Self {
        Self {
            width,
            open: false,
            x_offset: 0.0,
            moving: false,
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> bool {
        if input.is_pressed(InputCode::Grave) {
            self.toggle();
        }

        if self.moving {
            let sign = match self.open {
                true => 1.0,
                false => -1.0,
            };
            self.x_offset = (self.x_offset as f32 + (700.0 * dt * sign)).clamp(0.0, 255.0);

            if self.x_offset >= self.width as f32 || self.x_offset <= 0.0 {
                self.moving = false;
            }
        }

        self.open
    }

    pub fn blit(&mut self, buffer: &mut B2DS) {
        if self.x_offset <= 0.0 {
            return;
        }

        buffer.blit_fill((0, 0), (self.x_offset as i32, buffer.height), CONSOLE_COLOR);
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.moving = true;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
