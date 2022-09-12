use cgmath::{Vector3, VectorSpace};

use super::math::{Vertex, distance2D};

pub struct Renderer {
    scanlines_min: Vec<i16>,
    scanlines_max: Vec<i16>,
    scanlines_min_color: Vec<Vector3<f32>>,
    scanlines_max_color: Vec<Vector3<f32>>,
    width: i16,
    height: i16
}

impl Renderer {
    pub fn new(width: i16, height: i16) -> Renderer {
        let size = height as usize;
        let black = Vector3::new(0.0,0.0,0.0);
        Renderer {
            scanlines_min: vec![0; size],
            scanlines_max: vec![0; size],
            scanlines_min_color: vec![black; size],
            scanlines_max_color: vec![black; size],
            width,
            height }
    }

    pub fn draw_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex, frame: &mut [u8]) {
        let min_y = (v1.pos.y.min(v2.pos.y).min(v0.pos.y)) as usize;
        let max_y = (v1.pos.y.max(v2.pos.y).max(v0.pos.y)) as usize;

        for y in min_y..max_y {
            self.scanlines_min[y] = i16::MAX;
            self.scanlines_max[y] = i16::MIN;
        }

        self.calculate_line(v0, v1);
        self.calculate_line(v1, v2);
        self.calculate_line(v2, v0);

        for y in min_y..max_y {
            let min_x = self.scanlines_min[y];
            let max_x = self.scanlines_max[y];
            let min_color = self.scanlines_min_color[y];
            let max_color = self.scanlines_max_color[y];

            for x in min_x..max_x {
                let normalized = (x as f32 - min_x as f32) / (max_x as f32 - min_x as f32);
                let index = (y as usize * self.width as usize + x as usize) * 4;
                let color = min_color.lerp(max_color, normalized);

                frame[index] = (color.x * 255.0) as u8;
                frame[index+1] = (color.y * 255.0) as u8;
                frame[index+2] = (color.z * 255.0) as u8;
                frame[index+3] = u8::MAX;
            }
        }
    }

    fn calculate_line(&mut self, v0: Vertex, v1: Vertex) {
        let mut dx = v1.pos.x - v0.pos.x;
        let mut dy = v1.pos.y - v0.pos.y;
        let step = if dx.abs() >= dy.abs() { dx.abs() } else { dy.abs() };
        dx = dx / step;
        dy = dy / step;
        let mut x = v0.pos.x;
        let mut y = v0.pos.y;
        let distance = distance2D(v0.pos.x, v1.pos.x, v0.pos.y, v1.pos.y);
        let mut i = 1i16;
        loop {
            let lx = x as i16;
            let ly = y as usize;

            if lx < self.scanlines_min[ly] {
                self.scanlines_min[ly] = lx;

                let normalized = distance2D(v0.pos.x, x, v0.pos.y, y) / distance;
                self.scanlines_min_color[ly] = v0.color.lerp(v1.color, normalized);
            }
            if lx > self.scanlines_max[ly] {
                self.scanlines_max[ly] = lx;
                
                let normalized = distance2D(v0.pos.x, x, v0.pos.y, y) / distance;
                self.scanlines_max_color[ly] = v0.color.lerp(v1.color, normalized);
            }
    
            if i > step as i16 {
                break
            }
    
            x = x + dx;
            y = y + dy;
            i = i + 1;
        }
    }
}