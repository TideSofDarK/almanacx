pub mod text;
pub mod virtual_window;

use std::{
    mem,
    ops::{Deref, DerefMut},
};

use cgmath::Vector3;

use crate::utils::calculate_index;

use self::virtual_window::VirtualWindow;

pub trait B2DT: Deref<Target = [u8]> + DerefMut<Target = [u8]> {}
impl<T> B2DT for T where T: Deref<Target = [u8]> + DerefMut<Target = [u8]> {}

pub type B2DO = B2D<Vec<u8>>;
pub type B2DS<'a> = B2D<&'a mut [u8]>;

pub struct B2D<T: B2DT> {
    pub width: i32,
    pub height: i32,
    pub pixels: T,
}

impl B2DO {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width,
            height: height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.pixels.resize((width * height * 4) as usize, 0);
    }

    pub fn as_b2ds(&mut self) -> B2DS {
        B2DS {
            width: self.width,
            height: self.height,
            pixels: &mut self.pixels,
        }
    }
}

impl<T: B2DT> B2D<T> {
    pub fn get_color(&self, x: usize, y: usize) -> Vector3<u8> {
        let index = ((y * self.width as usize) + x) * 4;
        let channels = &self.pixels[index..index + 4];
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

    pub fn set_color(&mut self, x: i32, y: i32, c: &Vector3<u8>) {
        self.set_color_by_index(calculate_index(x, y, self.width) * 4, c);
    }

    pub fn set_color_by_index(&mut self, index: usize, c: &Vector3<u8>) {
        self.pixels[index] = c.x;
        self.pixels[index + 1] = c.y;
        self.pixels[index + 2] = c.z;
        self.pixels[index + 3] = u8::MAX;
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
                self.set_color(y, x, c);
            } else {
                self.set_color(x, y, c);
            }

            error += d_error;
            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= 2 * dx;
            }
        }
    }

    pub fn blit_buffer_full<A: B2DT>(&mut self, buffer: &B2D<A>, offset: (i32, i32)) {
        self.blit_full(
            &buffer.pixels,
            (buffer.width as i32, buffer.height as i32),
            offset,
        )
    }

    pub fn blit_full(&mut self, source: &[u8], source_size: (i32, i32), offset: (i32, i32)) {
        self.blit_region_copy(source, (0, 0), source_size, source_size.0, offset)
    }

    pub fn blit_region_copy(
        &mut self,
        source: &[u8],
        source_offset: (i32, i32),
        mut image_length: (i32, i32),
        source_width: i32,
        offset: (i32, i32),
    ) {
        self.blit_region(
            source,
            source_offset,
            image_length,
            source_width,
            offset,
            |dest, source| {
                dest.copy_from_slice(source);
            },
        )
    }

    pub fn blit_region_alpha(
        &mut self,
        source: &[u8],
        source_offset: (i32, i32),
        mut image_length: (i32, i32),
        source_width: i32,
        offset: (i32, i32),
    ) {
        self.blit_region(
            source,
            source_offset,
            image_length,
            source_width,
            offset,
            |dest, source| {
                dest.iter_mut().zip(source).for_each(|(d, s)| *d = *d | *s);
            },
        )
    }

    pub fn blit_region(
        &mut self,
        source: &[u8],
        source_offset: (i32, i32),
        mut image_length: (i32, i32),
        source_width: i32,
        offset: (i32, i32),
        method: fn(&mut [u8], &[u8]),
    ) {
        let mut source_offset_x = source_offset.0;
        if offset.0 < 0 {
            source_offset_x -= offset.0;
            image_length.0 += offset.0;
        }
        image_length.0 = image_length.0.min(self.width - offset.0);
        if image_length.0 <= 0 {
            return;
        }

        let mut source_offset_y = source_offset.1;
        if offset.1 < 0 {
            source_offset_y -= offset.1;
            image_length.1 += offset.1;
        }
        image_length.1 = image_length.1.min(self.height - offset.1);
        if image_length.1 <= 0 {
            return;
        }

        let slice_length = image_length.0 as usize * 4;

        let dest_offset_x = offset.0.max(0);
        let dest_offset_y = offset.1.max(0);

        for y in 0..image_length.1 {
            let dest_index = calculate_index(dest_offset_x, y + dest_offset_y, self.width) * 4;
            let source_index =
                calculate_index(source_offset_x, y + source_offset_y, source_width) * 4;

            method(
                &mut self.pixels[dest_index..dest_index + slice_length],
                &source[source_index..source_index + slice_length],
            );
        }
    }
}
