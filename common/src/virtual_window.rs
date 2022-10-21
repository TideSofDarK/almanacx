use cgmath::Vector3;

use crate::buffer2d::{Buffer2D, Buffer2DSlice};

pub struct VirtualWindow {
    pub x: i32,
    pub y: i32,
    pub buffer: Buffer2D,
}

impl VirtualWindow {
    pub fn new(x: i32, y: i32, width: usize, height: usize) -> Self {
        Self {
            x: x,
            y: y,
            buffer: Buffer2D::new(width, height, vec![0; width * height * 4]),
        }
    }

    pub fn get_buffer_slice(&mut self) -> Buffer2DSlice {
        let width = self.buffer.width as u32;
        let height = self.buffer.height as u32;
        Buffer2DSlice::new(width, height, &mut self.buffer.colors)
    }
}
