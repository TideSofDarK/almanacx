use std::mem;

use cgmath::Vector3;

pub struct DrawTarget<'a> {
    color_buffer: &'a mut [u8],
    width: i32,
    height: i32,
}

impl<'c, 'z> DrawTarget<'c> {
    pub fn new(color_buffer: &'c mut [u8], width: u32, height: u32) -> Self {
        Self {
            color_buffer: color_buffer,
            width: width as i32,
            height: height as i32,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.color_buffer.fill(0x00)
    }

    #[inline]
    pub fn get_width_f(&self) -> f32 {
        self.width as f32
    }

    #[inline]
    pub fn get_height_f(&self) -> f32 {
        self.height as f32
    }

    #[inline]
    pub fn calculate_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            return Some((y * self.width + x) as usize);
        }
        None
    }

    pub fn set_color_xy(&mut self, x: i32, y: i32, c: &Vector3<u8>) {
        if let Some(index) = self.calculate_index(x, y) {
            self.set_color_by_index(index * 4, c);
        }
    }

    pub fn set_color_by_index(&mut self, index: usize, c: &Vector3<u8>) {
        self.color_buffer[index] = c.x;
        self.color_buffer[index + 1] = c.y;
        self.color_buffer[index + 2] = c.z;
        self.color_buffer[index + 3] = u8::MAX;
    }

    pub fn draw_line_2d(&mut self, p0: Vector3<f32>, p1: Vector3<f32>, c: &Vector3<u8>) {
        let mut x0 = p0.x as i32;
        let mut x1 = p1.x as i32;
        let mut y0 = p0.y as i32;
        let mut y1 = p1.y as i32;
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
}
