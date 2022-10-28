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

pub struct Console {
    width: i32,
    font: Font,

    is_open: bool,
    current_width: f32,
    is_moving: bool,

    output_next_y: i32,
    output_buffer: B2DO,
    input_y: i32,
    input_buffer: B2DO,
}

impl Console {
    pub fn new(width: i32, height: i32, font: Font) -> Self {
        let input_height = font.glyph_size.0 + font.glyph_size.1;

        let output_next_y = font.glyph_size.0;
        let output_height = height - input_height;
        let mut buffer_output = B2DO::new(width, output_height);
        buffer_output.bitmap.fill(CONSOLE_COLOR);

        let input_y = height - font.glyph_size.0 - font.glyph_size.1;
        let mut buffer_input = B2DO::new(width, input_height);
        buffer_input.bitmap.fill(CONSOLE_COLOR);
        blit_char(&font, &mut buffer_input, ']', (0, 0));

        Self {
            width,
            font,

            is_open: false,
            current_width: 0.0,
            is_moving: false,

            output_next_y,
            output_buffer: buffer_output,
            input_y,
            input_buffer: buffer_input,
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
        if input.is_pressed(InputCode::Grave) {
            self.toggle();
        }

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
