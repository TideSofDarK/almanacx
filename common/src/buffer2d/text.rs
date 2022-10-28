use std::str::Chars;

use crate::buffer2d::{B2D, B2DO, B2DT};

const CHARS_FIRST: usize = 0x20;
const CHARS_LAST: usize = 0x7f;
const CHARS_COUNT: usize = CHARS_LAST - CHARS_FIRST;
const CHARS_SPACE: usize = CHARS_FIRST;
const CHARS_LB: usize = '\n' as usize;

pub struct Glyph(Vec<(i32, i32, u16)>);

pub struct Font {
    pub glyphs: Vec<Glyph>,
    pub glyph_size: (i32, i32),
}

impl Font {
    pub fn new(bitmap: B2DO, glyph_size: (i32, i32), offset_x: i32, offset_y: i32) -> Self {
        let len = bitmap.width / glyph_size.0;
        let mut glyphs = Vec::with_capacity(CHARS_COUNT as usize);
        for u in CHARS_FIRST..CHARS_LAST {
            let u = (u - CHARS_FIRST) as i32;
            let glyph_x_offset = ((u % len) + offset_x) * glyph_size.0;
            let glyph_y_offset = ((u / len) + offset_y) * glyph_size.1;

            let mut glyph = Glyph(Vec::new());

            for glyph_x in 0..glyph_size.0 {
                for glyph_y in 0..glyph_size.1 {
                    let color = bitmap.get_color(
                        glyph_x_offset as usize + glyph_x as usize,
                        glyph_y_offset as usize + glyph_y as usize,
                    );
                    if color == 0 {
                        continue;
                    }
                    glyph.0.push((glyph_x, glyph_y, color));
                }
            }

            glyphs.push(glyph);
        }

        Self { glyphs, glyph_size }
    }
}

pub fn blit_str_wrap<T: B2DT>(
    font: &Font,
    dest: &mut B2D<T>,
    string: &str,
    offset: (i32, i32),
    wrap_new_line_spaces: i32,
    scroll: bool,
) -> i32 {
    let mut col = 0;

    let mut dest_y = offset.1;

    let max_width = dest.width - offset.0;

    'words: for word in string.split(' ') {
        let word_width = word.len() as i32 * font.glyph_size.0;
        let mut char_wrap = false;
        if word_width > max_width {
            char_wrap = true;
        } else if max_width - (col * font.glyph_size.0) - word_width <= 0 {
            col = wrap_new_line_spaces;
            dest_y += font.glyph_size.1;
        }
        for c in word.chars() {
            if c as usize == CHARS_LB {
                col = wrap_new_line_spaces;
                dest_y += font.glyph_size.1;
                continue;
            }
            let mut dest_x = offset.0 + (col * font.glyph_size.0);
            if char_wrap && dest_x > dest.width - font.glyph_size.0 {
                col = wrap_new_line_spaces;
                dest_y += font.glyph_size.1;
                if c as usize == CHARS_SPACE {
                    continue;
                }
                dest_x = offset.0 + (col * font.glyph_size.0);
            }

            if dest_y + font.glyph_size.1 > dest.height {
                if scroll {
                    let scroll_amount = dest_y + font.glyph_size.1 - dest.height;
                    let scroll_rows =
                        (scroll_amount as f32 / font.glyph_size.1 as f32).ceil() as i32;
                    dest_y = dest_y - scroll_amount;

                    let rotate_length = (dest.width * scroll_amount) as usize;
                    dest.bitmap.rotate_left(rotate_length);
                    dest.bitmap[((dest.width * dest.height) as usize - rotate_length)..].fill(0);
                } else {
                    break 'words;
                }
            }
            blit_char(font, dest, c, (dest_x, dest_y));
            col += 1;
        }
        col += 1;
    }

    return dest_y - offset.1;
}

pub fn blit_str<T: B2DT>(font: &Font, dest: &mut B2D<T>, s: &str, offset: (i32, i32)) {
    let mut col = 0;
    for c in s.chars() {
        let dest_x = offset.0 + (col * font.glyph_size.0);
        if dest_x > dest.width - font.glyph_size.0 {
            return;
        }
        blit_char(font, dest, c, (dest_x, offset.1));
        col += 1;
    }
}

pub fn blit_char<T: B2DT>(font: &Font, dest: &mut B2D<T>, c: char, offset: (i32, i32)) {
    let u = c as usize;
    if u <= CHARS_FIRST || u > CHARS_LAST {
        return;
    }

    let u = u - CHARS_FIRST;
    let glyph = &font.glyphs[u];

    for glyph_pixel in glyph.0.iter() {
        dest.set_color(
            glyph_pixel.0 + offset.0,
            glyph_pixel.1 + offset.1,
            glyph_pixel.2,
        );
    }
}
