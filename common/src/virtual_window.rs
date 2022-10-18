use cgmath::Vector3;

use crate::buffer2d::{Buffer2D, Buffer2DSlice};

pub struct VirtualWindow {
    x: i32,
    y: i32,
    image: Buffer2D,
}

impl VirtualWindow {
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            x: x,
            y: y,
            image: Buffer2D::new(width, height, vec![0; width * height * 4]),
        }
    }

    #[inline]
    pub fn get_x(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn get_y(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn get_image(&self) -> &Buffer2D {
        &self.image
    }

    pub fn get_buffer_slice(&mut self) -> Buffer2DSlice {
        let width = self.image.get_width() as u32;
        let height = self.image.get_height() as u32;
        Buffer2DSlice::new(width, height, self.image.get_buffer_mut())
    }
}
