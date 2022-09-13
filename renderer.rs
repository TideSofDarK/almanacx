use std::ops::MulAssign;

use cgmath::{Vector3, VectorSpace, Matrix4, Point3, Deg, SquareMatrix, Vector4};

use super::math::{Vertex, distance2D};

pub struct Renderer {
    scanlines_min: Vec<i16>,
    scanlines_max: Vec<i16>,
    scanlines_min_color: Vec<Vector3<f32>>,
    scanlines_max_color: Vec<Vector3<f32>>,
    width: i16,
    height: i16,
    view_proj_mat: Matrix4<f32>,
    view_mat: Matrix4<f32>,
    proj_mat: Matrix4<f32>,
    viewport_mat: Matrix4<f32>
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
            height,
            view_mat: Matrix4::look_at_rh(
                Point3::new(5.0, 5.0, 3.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_z(),
            ),
            proj_mat: cgmath::perspective( Deg(115.0f32), width as f32 / height as f32, 0.1, 1000.0),
            viewport_mat: Matrix4{
                x: Vector4::new(width as f32/2.0, 0.0, 0.0, 0.0),
                y: Vector4::new(0.0, -height as f32/2.0, 0.0, 0.0),
                z: Vector4::new(0.0,0.0,1.0,0.0),
                w: Vector4::new(width as f32/2.0,height as f32/2.0,0.0,1.0)
            },
            view_proj_mat: Matrix4::identity()
        }
    }

    pub fn set_position (&mut self, pos: Vector3<f32>) {
        self.view_mat = Matrix4::look_to_rh(Point3::new(-pos.x, 0.0, -pos.y),
        Vector3::unit_z(),
        Vector3::unit_y());
    }

    pub fn begin(&mut self) {
        self.view_proj_mat = self.proj_mat * self.view_mat;
    }

    fn preprocess_triangle(&mut self, v0w: Vertex, v1w: Vertex, v2w: Vertex) -> Option<(Vertex, Vertex, Vertex)> {
        let mut v0 = Vertex{ pos: self.view_proj_mat * v0w.pos, color: v0w.color };
        let mut v1 = Vertex{ pos: self.view_proj_mat * v1w.pos, color: v1w.color };
        let mut v2 = Vertex{ pos: self.view_proj_mat * v2w.pos, color: v2w.color };

        if ((v0.pos.x > v0.pos.w) || (v0.pos.x < -v0.pos.w))
		{
			return None
		}
		else if ((v0.pos.y > v0.pos.w) || (v0.pos.y < -v0.pos.w))
		{
			return None
		}
		else if ((v0.pos.z > v0.pos.w) || (v0.pos.z < -v0.pos.w))
		{
			return None
		}

        if ((v1.pos.x > v1.pos.w) || (v1.pos.x < -v1.pos.w))
		{
			return None
		}
		else if ((v1.pos.y > v1.pos.w) || (v1.pos.y < -v1.pos.w))
		{
			return None
		}
		else if ((v1.pos.z > v1.pos.w) || (v1.pos.z < -v1.pos.w))
		{
			return None
		}

        if ((v2.pos.x > v2.pos.w) || (v2.pos.x < -v2.pos.w))
		{
			return None
		}
		else if ((v2.pos.y > v2.pos.w) || (v2.pos.y < -v2.pos.w))
		{
			return None
		}
		else if ((v2.pos.z > v2.pos.w) || (v2.pos.z < -v2.pos.w))
		{
			return None
		}

        let reciprocal = 1.0 / v0.pos.w;
		v0.pos.x *= reciprocal;
		v0.pos.y *= reciprocal;
		v0.pos.z *= reciprocal;
		v0.pos.w = 1.0;
        v0.pos = self.viewport_mat * v0.pos;

        let reciprocal = 1.0 / v1.pos.w;
		v1.pos.x *= reciprocal;
		v1.pos.y *= reciprocal;
		v1.pos.z *= reciprocal;
		v1.pos.w = 1.0;
        v1.pos = self.viewport_mat * v1.pos;

        let reciprocal = 1.0 / v2.pos.w;
		v2.pos.x *= reciprocal;
		v2.pos.y *= reciprocal;
		v2.pos.z *= reciprocal;
		v2.pos.w = 1.0;
        v2.pos = self.viewport_mat * v2.pos;

        println!("{:?} {:?} {:?}", v0.pos, v1.pos, v2.pos);

        if v0.pos.x < 0.0 && v1.pos.x < 0.0 && v2.pos.x < 0.0 ||
            v0.pos.x >= self.width as f32 && v1.pos.x > self.width as f32 && v2.pos.x > self.width as f32 ||
            v0.pos.y < 0.0 && v1.pos.y < 0.0 && v2.pos.y < 0.0 ||
            v0.pos.y >= self.height as f32 && v1.pos.y > self.height as f32 && v2.pos.y > self.height as f32 {
            return None
        }
        return Some((v0, v1, v2))
    }

    pub fn draw_triangle(&mut self, v0w: Vertex, v1w: Vertex, v2w: Vertex, frame: &mut [u8]) {
        let processed = self.preprocess_triangle(v0w, v1w, v2w);
        if processed.is_none() {
            return;
        }
        let (v0, v1, v2) = processed.unwrap();

        let min_y = (v1.pos.y.min(v2.pos.y).min(v0.pos.y)).max(0.0) as usize;
        let max_y = (v1.pos.y.max(v2.pos.y).max(v0.pos.y)).min(self.height as f32 - 1.0) as usize;

        for y in min_y..max_y {
            self.scanlines_min[y] = i16::MAX;
            self.scanlines_max[y] = i16::MIN;
        }

        self.calculate_line(v0, v1);
        self.calculate_line(v1, v2);
        self.calculate_line(v2, v0);

        for y in min_y..max_y {
            let min_x = self.scanlines_min[y].max(0);
            let max_x = self.scanlines_max[y].min(self.width-1);
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