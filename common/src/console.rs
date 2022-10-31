use crate::{
    buffer2d::{
        text::{blit_char, blit_str_wrap, Font},
        B2DO, B2DS,
    },
    platform::input::{Input, InputCode},
    utils::color_from_tuple,
};

const CONSOLE_COLOR: u16 = color_from_tuple((0, 0, 0));
const CONSOLE_LINE_SPACING: i32 = 4;
const CONSOLE_INPUT_CAPACITY: usize = 64;

pub struct Console {
    width: i32,
    font: Font,

    is_open: bool,
    current_width: f32,
    is_moving: bool,

    output_next_y: i32,
    output_buffer: B2DO,
    input_y: i32,
    input_char_x: i32,
    input_buffer: B2DO,
    input_string: String,
}

impl Console {
    pub fn new(width: i32, height: i32, font: Font) -> Self {
        let input_height = font.glyph_size.0 + font.glyph_size.1;

        let output_next_y = font.glyph_size.0;
        let output_height = height - input_height;
        let mut output_buffer = B2DO::new(width, output_height);
        output_buffer.bitmap.fill(CONSOLE_COLOR);

        let input_char_x = 0;
        let input_y = height - font.glyph_size.0 - font.glyph_size.1;
        let mut input_buffer = B2DO::new(width, input_height);
        input_buffer.bitmap.fill(CONSOLE_COLOR);
        blit_char(&font, &mut input_buffer, ']', (0, 0));

        Self {
            width,
            font,

            is_open: false,
            current_width: 0.0,
            is_moving: false,

            output_next_y,
            output_buffer,
            input_y,
            input_char_x,
            input_buffer,
            input_string: String::with_capacity(CONSOLE_INPUT_CAPACITY),
        }
    }

    pub fn put_string(&mut self, string: String) {
        self.put_line(string.as_str());
    }

    pub fn put_line(&mut self, text: &str) {
        let offset_y = blit_str_wrap(
            &self.font,
            &mut self.output_buffer,
            text,
            (self.font.glyph_size.0, self.output_next_y),
            2,
            true,
        );
        self.output_next_y += offset_y + self.font.glyph_size.1 + CONSOLE_LINE_SPACING;
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> bool {
        if self.is_moving {
            let sign = match self.is_open {
                true => 1.0,
                false => -1.0,
            };
            self.current_width =
                (self.current_width as f32 + (700.0 * dt * sign)).clamp(0.0, self.width as f32);

            if self.current_width >= self.width as f32 || self.current_width <= 0.0 {
                self.is_moving = false;
            }
        }

        if input.is_pressed(InputCode::Grave) {
            self.toggle();
        } else if self.is_open {
            if input.is_pressed(InputCode::Back) {
                self.input_string.pop();
                self.input_buffer
                    .blit_fill((self.input_char_x, 0), self.font.glyph_size, 0);
                self.input_char_x -= self.font.glyph_size.0;
            } else if input.is_pressed(InputCode::Return) {
                if self.input_string.len() != 0 {
                    let command_string = std::mem::take(&mut self.input_string);
                    self.put_line(command_string.as_str());
                    self.input_buffer.bitmap.fill(0);
                    blit_char(&self.font, &mut self.input_buffer, ']', (0, 0));
                    self.input_char_x = 0;
                }
            } else if let Some(last_char) = input.last_char {
                self.input_string.push(last_char);
                blit_char(
                    &self.font,
                    &mut self.input_buffer,
                    last_char,
                    (self.input_char_x + self.font.glyph_size.0, 0),
                );
                self.input_char_x += self.font.glyph_size.0;
            }
        }

        self.is_open
    }

    pub fn blit(&mut self, buffer: &mut B2DS) {
        if self.current_width <= 0.0 {
            return;
        }

        buffer.blit_region_copy(
            &self.output_buffer.bitmap,
            (self.width - self.current_width as i32, 0),
            (self.current_width as i32, self.output_buffer.height),
            self.width,
            (0, 0),
        );
        buffer.blit_region_copy(
            &self.input_buffer.bitmap,
            (self.width - self.current_width as i32, 0),
            (self.current_width as i32, self.input_buffer.height),
            self.width,
            (0, self.input_y),
        );
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        self.is_moving = true;
    }
}
