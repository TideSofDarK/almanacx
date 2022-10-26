use crate::{
    buffer2d::{text::Font, B2DO, B2DS},
    platform::input::{Input, InputCode},
    utils::color_from_tuple,
};

const CONSOLE_COLOR: u16 = color_from_tuple((0, 0, 0));
const CONSOLE_LINE_SPACING: i32 = 4;

pub struct Console {
    width: i32,
    height: i32,
    open: bool,
    x_offset: f32,
    moving: bool,
    next_y: i32,
    buffer: B2DO,
}

impl Console {
    pub fn new(width: i32, height: i32) -> Self {
        let mut buffer = B2DO::new(width, height);
        buffer.bitmap.fill(CONSOLE_COLOR);

        Self {
            width,
            height,
            open: false,
            x_offset: 0.0,
            moving: false,
            next_y: 0,
            buffer,
        }
    }

    pub fn put_string(&mut self, string: String, font: &Font) {
        self.put_line(string.as_str(), font);
    }

    pub fn put_line(&mut self, text: &str, font: &Font) {
        let offset_y = font.blit_str_wrap(
            &mut self.buffer,
            text,
            font.glyph_size.0,
            self.next_y + font.glyph_size.1,
            2,
        );
        self.next_y += offset_y + font.glyph_size.1 + CONSOLE_LINE_SPACING;
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
            self.x_offset =
                (self.x_offset as f32 + (700.0 * dt * sign)).clamp(0.0, self.width as f32);

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

        buffer.blit_region_copy(
            &self.buffer.bitmap,
            (self.width - self.x_offset as i32, 0),
            (self.x_offset as i32, self.height),
            self.width,
            (0, 0),
        );
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.moving = true;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
