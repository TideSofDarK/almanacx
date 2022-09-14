use cgmath::{Vector3, VectorSpace, Matrix4, Deg, SquareMatrix, Vector4};

use super::math::{Vertex, distance2D};

pub struct Renderer {
    width: i16,
    height: i16,
    scanlines_min: Vec<i16>,
    scanlines_max: Vec<i16>,
    scanlines_min_color: Vec<Vector3<f32>>,
    scanlines_max_color: Vec<Vector3<f32>>,
    z_buffer: Vec<f32>,
    view_proj_mat: Matrix4<f32>,
    proj_mat: Matrix4<f32>,
    viewport_mat: Matrix4<f32>
}

impl Renderer {
    pub fn new(width: i16, height: i16) -> Renderer {
        let height_size = height as usize;
        let width_size = width as usize;

        let black = Vector3::new(0.0,0.0,0.0);

        Renderer {
            width: width,
            height: height,
            scanlines_min: vec![0; height_size],
            scanlines_max: vec![0; height_size],
            scanlines_min_color: vec![black; height_size],
            scanlines_max_color: vec![black; height_size],
            z_buffer: vec![0.0; height_size * width_size],
            proj_mat: cgmath::perspective( Deg(90.0f32), width as f32 / height as f32, 0.1, 1000.0),
            viewport_mat: Matrix4{
                x: Vector4::new(width as f32/2.0, 0.0, 0.0, 0.0),
                y: Vector4::new(0.0, -height as f32/2.0, 0.0, 0.0),
                z: Vector4::new(0.0,0.0,1.0,0.0),
                w: Vector4::new(width as f32/2.0,height as f32/2.0,0.0,1.0)
            },
            view_proj_mat: Matrix4::identity()
        }
    }

    pub fn begin(&mut self, view_mat: Matrix4<f32>) {
        self.view_proj_mat = self.proj_mat * view_mat;

        for y in self.z_buffer.iter_mut() {
            *y = f32::MIN
        }
    }

    fn preprocess_triangle(&mut self, vertices: &mut [Vertex; 3]) -> bool {
        for v in vertices.iter_mut() {
            v.pos = self.view_proj_mat * v.pos;

            if (v.pos.x > v.pos.w) || (v.pos.x < -v.pos.w) {
                return false
            } else if (v.pos.y > v.pos.w) || (v.pos.y < -v.pos.w) {
                return false
            } else if (v.pos.z > v.pos.w) || (v.pos.z < -v.pos.w) {
                return false
            }

            let reciprocal = 1.0 / v.pos.w;
            v.pos.x *= reciprocal;
            v.pos.y *= reciprocal;
            v.pos.z *= reciprocal;
            v.pos.w = 1.0;
            v.pos = self.viewport_mat * v.pos;
        }

        if vertices[0].pos.y > vertices[1].pos.y {
            vertices.swap(0, 1);
        }
        if vertices[0].pos.y > vertices[2].pos.y {
            vertices.swap(0, 2);
        }
        if vertices[1].pos.y > vertices[2].pos.y {
            vertices.swap(1, 2);
        }

        // println!("{:?} {:?} {:?}", v0.pos, v1.pos, v2.pos);

        // if v0.pos.x < 0.0 && v1.pos.x < 0.0 && v2.pos.x < 0.0 ||
        //     v0.pos.x >= self.width as f32 && v1.pos.x > self.width as f32 && v2.pos.x > self.width as f32 ||
        //     v0.pos.y < 0.0 && v1.pos.y < 0.0 && v2.pos.y < 0.0 ||
        //     v0.pos.y >= self.height as f32 && v1.pos.y > self.height as f32 && v2.pos.y > self.height as f32 {
        //     return None
        // }

        return true
    }

    pub fn draw_triangle(&mut self, v0w: Vertex, v1w: Vertex, v2w: Vertex, frame: &mut [u8]) {
        let mut vertices = [v0w, v1w, v2w];
        if !self.preprocess_triangle(&mut vertices) {
            return;
        }

        let min_y = (vertices[1].pos.y.min(vertices[2].pos.y).min(vertices[0].pos.y)).max(0.0) as usize;
        let max_y = (vertices[1].pos.y.max(vertices[2].pos.y).max(vertices[0].pos.y)).min(self.height as f32 - 1.0) as usize;

        for y in min_y..max_y {
            self.scanlines_min[y] = i16::MAX;
            self.scanlines_max[y] = i16::MIN;
        }

        self.calculate_line(vertices[0], vertices[1]);
        self.calculate_line(vertices[1], vertices[2]);
        self.calculate_line(vertices[2], vertices[0]);

        for y in min_y..max_y {
            let min_x = self.scanlines_min[y].max(0);
            let max_x = self.scanlines_max[y].min(self.width-1);
            let min_color = self.scanlines_min_color[y];
            let max_color = self.scanlines_max_color[y];

            for x in min_x..max_x {
                let normalized = (x as f32 - min_x as f32) / (max_x as f32 - min_x as f32);
                let index = (y as usize * self.width as usize + x as usize) * 4;
                let color = min_color.lerp(max_color, normalized);

                // let z = vertices[0].pos.z

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

            if ly < self.height as usize && ly > 0 {
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