use cgmath::Vector3;

use crate::draw_target::DrawTarget;

pub struct VirtualWindow {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color_buffer: Vec<u8>,
}

impl VirtualWindow {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x: x,
            y: y,
            width: width,
            height: height,
            color_buffer: vec![0; width * height * 4],
        }
    }

    pub fn get_draw_target(&mut self) -> DrawTarget {
        DrawTarget::new(
            self.color_buffer.as_mut_slice(),
            self.width as u32,
            self.height as u32,
        )
    }

    pub fn draw(&self, draw_target: &mut DrawTarget) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = ((y * self.width) + x) * 4;
                draw_target.set_color_xy(
                    (x + self.x) as i32,
                    (y + self.y) as i32,
                    &Vector3::new(
                        self.color_buffer[index],
                        self.color_buffer[index + 1],
                        self.color_buffer[index + 2],
                    ),
                )
            }
        }
    }
}
