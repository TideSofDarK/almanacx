use crate::buffer2d::{B2D, B2DO, B2DS, B2DT};

pub struct Font {
    pub bitmap: B2DO,
    pub char_size: i32,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Font {}

impl<T: B2DT> B2D<T> {
    pub fn blit_str(&mut self, s: &str, x: i32, y: i32, font: &Font) {
        let mut i = 0;
        for c in s.chars() {
            let u = c as usize;
            if u < 0x20 || u > 0x7f {
                continue;
            }
            if u != 0x20 {
                let u = (u - 0x20) as i32;
                let char_x_offset =
                    ((u % (font.bitmap.width / font.char_size)) + font.offset_x) * font.char_size;
                let char_y_offset =
                    ((u / (font.bitmap.width / font.char_size)) + font.offset_y) * font.char_size;

                for char_x in 0..font.char_size {
                    for char_y in 0..font.char_size {
                        let color = font.bitmap.get_color(
                            char_x_offset as usize + char_x as usize,
                            char_y_offset as usize + char_y as usize,
                        );
                        if color.x == 0 && color.y == 0 && color.z == 0 {
                            continue;
                        }
                        self.set_color_xy(char_x + (i * font.char_size) + x, char_y + y, &color);
                    }
                }
            }

            i += 1;
        }
    }
}
