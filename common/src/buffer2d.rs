use std::mem;

use cgmath::Vector3;

use crate::{
    utils::{blit_buffer_to_buffer, calculate_index},
    virtual_window::VirtualWindow,
};

pub struct Buffer2D {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<u8>,
}

impl Buffer2D {
    pub const fn new(width: usize, height: usize, colors: Vec<u8>) -> Self {
        Self {
            width: width,
            height: height,
            colors: colors,
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Vector3<u8> {
        let index = ((y * self.width as usize) + x) * 4;
        let channels = &self.colors[index..index + 4];
        Vector3 {
            x: channels[0],
            y: channels[1],
            z: channels[2],
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Vector3<u8> {
        let x = (u * (self.width as f32 - 1.0)).round() as usize;
        let y = (v * (self.height as f32 - 1.0)).round() as usize;

        self.get_color(x, y)
    }
}

pub struct Buffer2DSlice<'a> {
    pub width: i32,
    pub height: i32,
    colors: &'a mut [u8],
}

impl<'c> Buffer2DSlice<'c> {
    pub fn new(width: u32, height: u32, color_buffer: &'c mut [u8]) -> Self {
        Self {
            colors: color_buffer,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn clear(&mut self) {
        self.colors.fill(0x00)
    }

    pub fn set_color_xy(&mut self, x: i32, y: i32, c: &Vector3<u8>) {
        self.set_color_by_index(calculate_index(x, y, self.width) * 4, c);
    }

    pub fn set_color_by_index(&mut self, index: usize, c: &Vector3<u8>) {
        self.colors[index] = c.x;
        self.colors[index + 1] = c.y;
        self.colors[index + 2] = c.z;
        self.colors[index + 3] = u8::MAX;
    }

    pub fn draw_line_2d(&mut self, p0: Vector3<f32>, p1: Vector3<f32>, c: &Vector3<u8>) {
        let mut x0 = (p0.x as i32).clamp(0, self.width - 1);
        let mut x1 = (p1.x as i32).clamp(0, self.width - 1);
        let mut y0 = (p0.y as i32).clamp(0, self.height - 1);
        let mut y1 = (p1.y as i32).clamp(0, self.height - 1);
        let mut z0 = p0.z;
        let mut z1 = p1.z;

        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
            mem::swap(&mut z0, &mut z1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let dz = if x1 == x0 {
            0.0
        } else {
            (z1 - z0) as f32 / (x1 - x0) as f32
        };

        let mut error = 0;
        let d_error = 2 * dy.abs();

        let mut y = y0;
        let mut z = z0;

        for x in x0..x1 {
            z += dz;
            if steep {
                self.set_color_xy(y, x, c);
            } else {
                self.set_color_xy(x, y, c);
            }

            error += d_error;
            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= 2 * dx;
            }
        }
    }

    pub fn blit_virtual_window(&mut self, virtual_window: &VirtualWindow) {
        self.blit(
            &virtual_window.buffer.colors,
            virtual_window.buffer.width as i32,
            virtual_window.buffer.height as i32,
            virtual_window.x,
            virtual_window.y,
        )
    }

    pub fn blit_buffer(&mut self, buffer: &Buffer2D, offset_x: i32, offset_y: i32) {
        self.blit(
            &buffer.colors,
            buffer.width as i32,
            buffer.height as i32,
            offset_x,
            offset_y,
        )
    }

    fn blit(
        &mut self,
        source: &[u8],
        source_width: i32,
        source_height: i32,
        offset_x: i32,
        offset_y: i32,
    ) {
        blit_buffer_to_buffer(
            self.colors,
            self.width,
            self.height,
            source,
            source_width,
            source_height,
            offset_x,
            offset_y,
        )
    }
}
