use std::{cell::RefCell, rc::Rc};

use super::{B2D, B2DO, B2DT};

pub struct VirtualWindow {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub minimized: bool,
    pub buffer: Rc<RefCell<B2DO>>,
}

impl VirtualWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: x,
            y: y,
            z: 0,
            minimized: false,
            buffer: Rc::new(RefCell::new(B2DO::new(width, height))),
        }
    }

    pub fn blit_with_border<T: B2DT>(&mut self, dest: &mut B2D<T>, border_texture: &B2DO) {
        let buffer = self.buffer.borrow();
        dest.blit_full(
            &buffer.pixels,
            (buffer.width, buffer.height),
            (self.x, self.y),
        );

        let border_padding = 1;
        let border_size = (border_texture.width - border_padding) / 2;
        let border_offset = 6;

        // Top left
        dest.blit_region_alpha(
            &border_texture.pixels,
            (0, 0),
            (border_size, border_size),
            border_texture.width,
            (self.x - border_offset, self.y - border_offset),
        );

        // Middle
        for x in (self.x - border_offset + border_size)
            ..(self.x + buffer.width - border_size + border_offset)
        {
            dest.blit_region_alpha(
                &border_texture.pixels,
                (border_size + border_padding, 0),
                (border_padding, border_size),
                border_texture.width,
                (x, self.y - border_offset),
            );

            dest.blit_region_alpha(
                &border_texture.pixels,
                (border_size + border_padding, border_size + border_padding),
                (border_padding, border_size),
                border_texture.width,
                (x, self.y + buffer.height + border_offset - border_size),
            );
        }

        // Top right
        dest.blit_region_alpha(
            &border_texture.pixels,
            (border_size + border_padding, 0),
            (border_size, border_size),
            border_texture.width,
            (
                self.x + buffer.width - border_size + border_offset,
                self.y - border_offset,
            ),
        );

        // Bottom left
        dest.blit_region_alpha(
            &border_texture.pixels,
            (0, border_size + border_padding),
            (border_size, border_size),
            border_texture.width,
            (
                self.x - border_offset,
                self.y + buffer.height - border_size + border_offset,
            ),
        );

        // Bottom right
        dest.blit_region_alpha(
            &border_texture.pixels,
            (border_size + border_padding, border_size + border_padding),
            (border_size, border_size),
            border_texture.width,
            (
                self.x + buffer.width - border_size + border_offset,
                self.y + buffer.height - border_size + border_offset,
            ),
        );

        // Left and right
        for y in (self.y - border_offset + border_size)
            ..(self.y + buffer.height - border_size + border_offset)
        {
            dest.blit_region_alpha(
                &border_texture.pixels,
                (0, border_size + border_padding),
                (border_size, border_padding),
                border_texture.width,
                (self.x - border_offset, y),
            );

            dest.blit_region_alpha(
                &border_texture.pixels,
                (border_size + border_padding, border_size + border_padding),
                (border_size, border_padding),
                border_texture.width,
                (self.x + buffer.width + border_offset - border_size, y),
            );
        }
    }
}

impl Default for VirtualWindow {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            minimized: Default::default(),
            buffer: Rc::new(RefCell::new(B2DO::new(
                Default::default(),
                Default::default(),
            ))),
        }
    }
}
