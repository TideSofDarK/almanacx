use cgmath::{
    Deg, InnerSpace, Matrix3, Matrix4, SquareMatrix, Vector2, Vector3, Vector4, Zero,
};

use crate::{is_between, math::barycentric, wad::TextureData};

use super::math::Vertex;

enum FrustumClipMask {
    PositiveX = 1 << 0,
    NegativeX = 1 << 1,
    PositiveY = 1 << 2,
    NegativeY = 1 << 3,
    PositiveZ = 1 << 4,
    NegativeZ = 1 << 5
}

pub struct Renderer {
    width: usize,
    height: usize,
    width_f: f32,
    height_f: f32,
    z_buffer: Vec<f32>,
    view_proj_mat: Matrix4<f32>,
    proj_mat: Matrix4<f32>,
    viewport_mat: Matrix4<f32>,
    texture: Option<TextureData>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let width_f = width as f32;
        let height_f = height as f32;

        let height_size = height as usize;
        let width_size = width as usize;

        let black = Vector3::new(0.0, 0.0, 0.0);

        Self {
            width: width as usize,
            height: height as usize,
            width_f: width_f,
            height_f: height_f,
            z_buffer: vec![0.0; height_size * width_size],
            proj_mat: cgmath::perspective(Deg(90.0f32), width_f / height_f, 0.01, 1000.0),
            viewport_mat: Matrix4 {
                x: Vector4::new(width_f / 2.0, 0.0, 0.0, 0.0),
                y: Vector4::new(0.0, -height_f / 2.0, 0.0, 0.0),
                z: Vector4::new(0.0, 0.0, 1.0, 0.0),
                w: Vector4::new(width_f / 2.0, height_f / 2.0, 0.0, 1.0),
            },
            view_proj_mat: Matrix4::identity(),
            texture: None,
        }
    }

    pub fn begin(&mut self, view_mat: Matrix4<f32>) {
        self.view_proj_mat = self.proj_mat * view_mat;
        self.z_buffer.fill(f32::MAX);
    }

    pub fn set_texture(&mut self, texture_data: TextureData) {
        self.texture = Some(texture_data);
    }

    pub fn draw_gizmo(&mut self, vw: Vertex, frame: &mut [u8]) {
        let mut v = vw;

        v.pos = self.view_proj_mat * v.pos;

        if (v.pos.z > v.pos.w) || (v.pos.z < -v.pos.w) {
            return;
        }

        v.pos = self.viewport_mat * v.pos;

        v.pos.x /= v.pos.w;
        v.pos.y /= v.pos.w;
        v.pos.z /= v.pos.w;
        v.pos.w = 1.0;

        let index = (v.pos.y as usize * self.width as usize + v.pos.x as usize) * 4;

        frame[index] = (v.color.x * 255.0) as u8;
        frame[index + 1] = (v.color.y * 255.0) as u8;
        frame[index + 2] = (v.color.z * 255.0) as u8;
        frame[index + 3] = u8::MAX;
    }

    pub fn draw_line(&mut self, v0w: Vertex, v1w: Vertex, frame: &mut [u8]) {
        let mut vs = [v0w, v1w];

        for i in 0..2 {
            vs[i].pos = self.view_proj_mat * vs[i].pos;

            vs[i].pos = self.viewport_mat * vs[i].pos;

            vs[i].pos.x /= vs[i].pos.w;
            vs[i].pos.y /= vs[i].pos.w;
            vs[i].pos.z /= vs[i].pos.w;
            vs[i].pos.w = 1.0;
        }

        for i in 0..2 {

        }

        let mut dx = vs[1].pos.x - vs[0].pos.x;
        let mut dy = vs[1].pos.y - vs[0].pos.y;
        let step = if dx.abs() >= dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };
        dx /= step;
        dy /= step;
        let mut x = vs[0].pos.x;
        let mut y = vs[0].pos.y;
        let mut i = 1i16;
        loop {
            let lx = x as usize;
            let ly = y as usize;

            let index = (ly as usize * self.width as usize + lx as usize) * 4;

            frame[index] = (255.0) as u8;
            frame[index + 1] = (255.0) as u8;
            frame[index + 2] = (255.0) as u8;
            frame[index + 3] = u8::MAX;

            if i > step as i16 {
                break;
            }

            x += dx;
            y += dy;
            i += 1;
        }
    }

    pub fn draw_triangle(&mut self, v0w: &Vertex, v1w: &Vertex, v2w: &Vertex, frame: &mut [u8]) {
        let mut vs_clip = [*v0w, *v1w, *v2w];
        let mut pos_viewport = [Vector4::zero(), Vector4::zero(), Vector4::zero()];
        let mut pos_perspective = [Vector3::zero(), Vector3::zero(), Vector3::zero()];

        let mut completely_obscured = true;

        for i in 0..3 {
            vs_clip[i].pos = self.view_proj_mat * vs_clip[i].pos;

            if (vs_clip[i].pos.z > vs_clip[i].pos.w) || (vs_clip[i].pos.z < -vs_clip[i].pos.w) {
                return;
            }

            pos_viewport[i] = self.viewport_mat * vs_clip[i].pos;

            pos_perspective[i].x = pos_viewport[i].x / pos_viewport[i].w;
            pos_perspective[i].y = pos_viewport[i].y / pos_viewport[i].w;
            pos_perspective[i].z = pos_viewport[i].z / pos_viewport[i].w;

            if completely_obscured
                && is_between!(pos_perspective[i].x, 0.0, self.width_f)
                && is_between!(pos_perspective[i].y, 0.0, self.height_f)
            {
                completely_obscured = false;
            }
        }

        if completely_obscured {
            return;
        }

        let has_uv = vs_clip.iter().all(|v| (v.uv.is_some()));

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for i in &pos_perspective {
            min_x = min_x.min(i.x);
            max_x = max_x.max(i.x);
            min_y = min_y.min(i.y);
            max_y = max_y.max(i.y);
        }
        let min_x = (min_x - 1.0).clamp(0.0, self.width_f - 1.0) as usize;
        let max_x = (max_x + 1.0).clamp(0.0, self.width_f - 1.0) as usize;
        let min_y = (min_y - 1.0).clamp(0.0, self.height_f - 1.0) as usize;
        let max_y = (max_y + 1.0).clamp(0.0, self.height_f - 1.0) as usize;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let index = y * self.width + x;

                let bc_screen = barycentric(&pos_perspective, Vector2::new(x as f32, y as f32));
                let mut bc_clip = Vector3::new(
                    bc_screen.x / pos_viewport[0].w,
                    bc_screen.y / pos_viewport[1].w,
                    bc_screen.z / pos_viewport[2].w,
                );
                bc_clip = bc_clip / (bc_clip.x + bc_clip.y + bc_clip.z);

                let frag_depth =
                    Vector3::new(vs_clip[0].pos.z, vs_clip[1].pos.z, vs_clip[2].pos.z).dot(bc_clip);

                if bc_screen.x < 0.0
                    || bc_screen.y < 0.0
                    || bc_screen.z < 0.0
                    || frag_depth > self.z_buffer[index]
                {
                    continue;
                }

                self.z_buffer[index] = frag_depth;

                let color = if has_uv && self.texture.is_some() {
                    let uv = Matrix3::from_cols(
                        vs_clip[0].uv.unwrap().extend(0.0),
                        vs_clip[1].uv.unwrap().extend(0.0),
                        vs_clip[2].uv.unwrap().extend(0.0),
                    ) * bc_clip;
                    let texture = self.texture.as_ref().unwrap();
                    let texture_x = (uv.x * (texture.width as f32 - 1.0)).round() as usize;
                    let texture_y = (uv.y * (texture.height as f32 - 1.0)).round() as usize;
                    texture.colors[texture_y * texture.width + texture_x]
                } else {
                    (Matrix3::from_cols(vs_clip[0].color, vs_clip[1].color, vs_clip[2].color)
                        * bc_clip)
                        .map(|c| ((c * 255.0) as u8))
                };

                let pixel_index = index * 4;
                frame[pixel_index] = color.x;
                frame[pixel_index + 1] = color.y;
                frame[pixel_index + 2] = color.z;
                frame[pixel_index + 3] = u8::MAX;
            }
        }
    }

    fn count_frustum_clip_mask(v: &Vector4<f32>) -> i32 {
        let mut mask = 0;
        if v.w < v.x {mask |= FrustumClipMask::PositiveX as i32;}
        if v.w < -v.x {mask |= FrustumClipMask::NegativeX as i32;}
        if v.w < v.y {mask |= FrustumClipMask::PositiveY as i32;}
        if v.w < -v.y {mask |= FrustumClipMask::NegativeY as i32;}
        if v.w < v.z {mask |= FrustumClipMask::PositiveZ as i32;}
        if v.w < -v.z {mask |= FrustumClipMask::NegativeZ as i32;}
        mask
    }
}
