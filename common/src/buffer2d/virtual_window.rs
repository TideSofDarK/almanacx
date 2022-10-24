use std::{cell::RefCell, rc::Rc};

use super::{B2D, B2DO, B2DT};

pub struct WindowBorder {
    pub padding: i32,
    pub size: i32,
    pub offset: i32,
    pub texture: B2DO,
}

impl WindowBorder {
    pub fn new(texture: B2DO) -> Self {
        Self {
            padding: 1,
            size: (texture.width - 1) / 2,
            offset: 6,
            texture: texture,
        }
    }
}

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

    pub fn blit_with_border<T: B2DT>(&mut self, dest: &mut B2D<T>, border: &WindowBorder) {
        let buffer = self.buffer.borrow();
        dest.blit_full(
            &buffer.pixels,
            (buffer.width, buffer.height),
            (self.x, self.y),
        );

        // Top left
        dest.blit_region_alpha(
            &border.texture.pixels,
            (0, 0),
            (border.size, border.size),
            border.texture.width,
            (self.x - border.offset, self.y - border.offset),
        );

        // Middle
        for x in (self.x - border.offset + border.size)
            ..(self.x + buffer.width - border.size + border.offset)
        {
            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, 0),
                (border.padding, border.size),
                border.texture.width,
                (x, self.y - border.offset),
            );

            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, border.size + border.padding),
                (border.padding, border.size),
                border.texture.width,
                (x, self.y + buffer.height + border.offset - border.size),
            );
        }

        // Top right
        dest.blit_region_alpha(
            &border.texture.pixels,
            (border.size + border.padding, 0),
            (border.size, border.size),
            border.texture.width,
            (
                self.x + buffer.width - border.size + border.offset,
                self.y - border.offset,
            ),
        );

        // Bottom left
        dest.blit_region_alpha(
            &border.texture.pixels,
            (0, border.size + border.padding),
            (border.size, border.size),
            border.texture.width,
            (
                self.x - border.offset,
                self.y + buffer.height - border.size + border.offset,
            ),
        );

        // Bottom right
        dest.blit_region_alpha(
            &border.texture.pixels,
            (border.size + border.padding, border.size + border.padding),
            (border.size, border.size),
            border.texture.width,
            (
                self.x + buffer.width - border.size + border.offset,
                self.y + buffer.height - border.size + border.offset,
            ),
        );

        // Left and right
        for y in (self.y - border.offset + border.size)
            ..(self.y + buffer.height - border.size + border.offset)
        {
            dest.blit_region_alpha(
                &border.texture.pixels,
                (0, border.size + border.padding),
                (border.size, border.padding),
                border.texture.width,
                (self.x - border.offset, y),
            );

            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, border.size + border.padding),
                (border.size, border.padding),
                border.texture.width,
                (self.x + buffer.width + border.offset - border.size, y),
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
