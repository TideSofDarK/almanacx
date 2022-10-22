pub mod text;
pub mod virtual_window;

use std::{
    mem,
    ops::{Deref, DerefMut},
};

use cgmath::Vector3;

use crate::utils::calculate_index;

pub trait B2DT: Deref<Target = [u8]> + DerefMut<Target = [u8]> {}
impl<T> B2DT for T where T: Deref<Target = [u8]> + DerefMut<Target = [u8]> {}

pub type B2DO = B2D<Vec<u8>>;
pub type B2DS<'a> = B2D<&'a mut [u8]>;

pub struct B2D<T: B2DT> {
    pub width: i32,
    pub height: i32,
    pub colors: T,
}

impl<T: B2DT> B2D<T> {
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

    pub fn clear(&mut self) {
        self.colors.fill(0x00);
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

    pub fn blit_buffer<A: B2DT>(&mut self, buffer: &B2D<A>, offset_x: i32, offset_y: i32) {
        self.blit(
            &buffer.colors,
            buffer.width as i32,
            buffer.height as i32,
            offset_x,
            offset_y,
        )
    }

    pub fn blit(
        &mut self,
        source: &[u8],
        source_width: i32,
        source_height: i32,
        offset_x: i32,
        offset_y: i32,
    ) {
        let mut source_offset_x = 0;
        if offset_x < 0 {
            source_offset_x = offset_x.abs();
        }
        let mut image_length_x = source_width - source_offset_x;
        image_length_x = image_length_x.min(self.width - offset_x);
        if image_length_x <= 0 {
            return;
        }

        let mut source_offset_y = 0;
        if offset_y < 0 {
            source_offset_y = offset_y.abs();
        }
        let mut image_length_y = source_height - source_offset_y;
        image_length_y = image_length_y.min(self.height - offset_y);
        if image_length_y <= 0 {
            return;
        }

        let slice_length = image_length_x as usize * 4;

        let dest_offset_x = offset_x.max(0);
        let dest_offset_y = offset_y.max(0);

        for y in 0..image_length_y {
            let dest_index = calculate_index(dest_offset_x, y + dest_offset_y, self.width) * 4;
            let source_index =
                calculate_index(source_offset_x, y + source_offset_y, source_width) * 4;
            self.colors[dest_index..dest_index + slice_length]
                .copy_from_slice(&source[source_index..source_index + slice_length])
        }
    }
}
