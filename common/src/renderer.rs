pub mod camera;
mod clipping;
mod texture;
pub mod utils;

use std::mem;

use cgmath::{InnerSpace, Matrix3, Matrix4, Vector2, Vector3, Vector4, VectorSpace, Zero};

use self::clipping::{
    clip_vertices, count_frustum_clip_mask, FRUSTUM_CLIP_MASK, FRUSTUM_CLIP_PLANE,
};

use super::{
    math::{max3, min3, orient2d},
    wad::TextureData,
};

pub struct Renderer {
    width: usize,
    height: usize,
    width_f: f32,
    height_f: f32,
    z_buffer: Vec<f32>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let width_f = width as f32;
        let height_f = height as f32;

        let height_size = height as usize;
        let width_size = width as usize;

        Self {
            width: width as usize,
            height: height as usize,
            width_f: width_f,
            height_f: height_f,
            z_buffer: vec![0.0; height_size * width_size],
        }
    }

    pub fn begin_3d<'a, 'b>(
        &'b mut self,
        proj_mat: &Matrix4<f32>,
        view_mat: &Matrix4<f32>,
        frame: &'a mut [u8],
        viewport: Vector4<f32>,
    ) -> RenderContext<'a, 'b> {
        RenderContext::new(
            proj_mat * view_mat,
            FrameBuffer {
                frame: frame,
                width: self.width as i32,
                height: self.height as i32,
            },
            &mut self.z_buffer,
            Vector4::new(
                viewport.z / 2.0,
                viewport.w / 2.0,
                viewport.x + viewport.z / 2.0,
                viewport.y + viewport.w / 2.0,
            ),
        )
    }
}

pub struct RenderContext<'a, 'b> {
    view_proj_mat: Matrix4<f32>,
    frame_buffer: FrameBuffer<'a>,
    z_buffer: &'b mut Vec<f32>,
    texture: Option<TextureData>,
    viewport: Vector4<f32>,
    tris_count: i32,
}

impl<'a, 'b> RenderContext<'a, 'b> {
    pub fn new(
        view_proj_mat: Matrix4<f32>,
        frame: FrameBuffer<'a>,
        z_buffer: &'b mut Vec<f32>,
        viewport: Vector4<f32>,
    ) -> Self {
        z_buffer.fill(f32::MAX);
        Self {
            view_proj_mat: view_proj_mat,
            frame_buffer: frame,
            z_buffer: z_buffer,
            texture: None,
            viewport: viewport,
            tris_count: 0,
        }
    }

    #[inline]
    fn perspective_division(&self, mut pos: Vector4<f32>) -> Vector4<f32> {
        let inv_w = 1.0 / pos.w;
        pos.x *= inv_w;
        pos.y *= inv_w;
        pos.z *= inv_w;

        pos
    }

    #[inline]
    fn transform_viewport(&self, mut pos: Vector4<f32>) -> Vector4<f32> {
        pos.x = pos.x * self.viewport.x as f32 + self.viewport.z;
        pos.y = pos.y * self.viewport.y as f32 + self.viewport.w;

        pos
    }

    pub fn set_texture(&mut self, texture_data: Option<TextureData>) {
        self.texture = texture_data;
    }

    pub fn take_texture(&mut self) -> Option<TextureData> {
        self.texture.take()
    }

    pub fn draw_gizmo(&mut self, vw: Vertex) {
        let mut v = vw;

        v.pos = self.view_proj_mat * v.pos;

        if (v.pos.z > v.pos.w) || (v.pos.z < -v.pos.w) {
            return;
        }

        // v.pos = self.renderer.get_viewport() * v.pos;

        self.perspective_division(v.pos);

        self.transform_viewport(v.pos);

        // v.pos.x /= v.pos.w;
        // v.pos.y /= v.pos.w;
        // v.pos.z /= v.pos.w;
        // v.pos.w = 1.0;

        self.frame_buffer.set_pixel_xy(
            v.pos.x as i32,
            v.pos.y as i32,
            &Vector3::new(
                (v.color.x * 255.0) as u8,
                (v.color.y * 255.0) as u8,
                (v.color.z * 255.0) as u8,
            ),
        )
    }

    pub fn draw_line(&mut self, mut p0: Vector4<f32>, mut p1: Vector4<f32>, c: Vector3<u8>) {
        p0 = self.view_proj_mat * p0;
        p1 = self.view_proj_mat * p1;

        let mask0 = count_frustum_clip_mask(p0);
        let mask1 = count_frustum_clip_mask(p1);

        let mut full_clip = false;
        let mut t0: f32 = 0.0;
        let mut t1: f32 = 1.0;

        let mask = mask0 | mask1;
        if mask != 0 {
            for i in 0..6 {
                if (mask & FRUSTUM_CLIP_MASK[i]) != 0 {
                    let d0 = FRUSTUM_CLIP_PLANE[i].dot(p0);
                    let d1 = FRUSTUM_CLIP_PLANE[i].dot(p1);

                    if d0 < 0.0 && d1 < 0.0 {
                        full_clip = true;
                        break;
                    } else if d0 < 0.0 {
                        let t = -d0 / (d1 - d0);
                        t0 = t0.max(t);
                    } else {
                        let t = d0 / (d0 - d1);
                        t1 = t1.min(t);
                    }
                }
            }
        }

        if full_clip {
            return;
        }

        if mask0 != 0 {
            p0 = p0.lerp(p1, t0);
        }

        if mask1 != 0 {
            p1 = p0.lerp(p1, t1);
        }

        p0 = self.perspective_division(p0);
        p1 = self.perspective_division(p1);

        p0 = self.transform_viewport(p0);
        p1 = self.transform_viewport(p1);

        self.frame_buffer.draw_line_2d(p0, p1, &c);
    }

    fn rasterize_triangle(&mut self, v0w: Vertex, v1w: Vertex, v2w: Vertex) {
        let vertices: [Vertex; 3] = [v0w, v1w, v2w];
        let mut pos_viewport = [Vector4::zero(), Vector4::zero(), Vector4::zero()];
        let mut pos_screen = [Vector4::zero(), Vector4::zero(), Vector4::zero()];

        for i in 0..3 {
            pos_viewport[i] = self.transform_viewport(vertices[i].pos);
            pos_screen[i] = self.transform_viewport(self.perspective_division(vertices[i].pos));

            // pos_screen[i] = self.perspective_division(pos_viewport[i]);

            // pos_viewport[i] = self.viewport_mat * vertices[i].pos;

            // pos_screen[i].x = (pos_viewport[i].x / pos_viewport[i].w) as i32;
            // pos_screen[i].y = (pos_viewport[i].y / pos_viewport[i].w) as i32;
            // pos_screen[i].z = pos_viewport[i].z / pos_viewport[i].w;
        }

        // self.frame_buffer
        //     .draw_line_2d(pos_screen[0], pos_screen[1], Vector3::new(255, 255, 255));
        // self.frame_buffer
        //     .draw_line_2d(pos_screen[1], pos_screen[2], Vector3::new(255, 255, 255));
        // self.frame_buffer
        //     .draw_line_2d(pos_screen[2], pos_screen[0], Vector3::new(255, 255, 255));

        let pos_screen = pos_screen.map(|pos| Vector2::new(pos.x as i32, pos.y as i32)); //[Vector4::zero(), Vector4::zero(), Vector4::zero()];

        println!(
            "{:?} {:?}",
            (pos_screen[1].x - pos_screen[0].x),
            (pos_screen[2].y - pos_screen[0].y)
        );
        // CW backface culling
        if (pos_screen[1].x - pos_screen[0].x) * (pos_screen[2].y - pos_screen[0].y)
            - (pos_screen[1].y - pos_screen[0].y) * (pos_screen[2].x - pos_screen[0].x)
            < 0
        {
            return;
        }

        let min_x = min3(pos_screen[0].x, pos_screen[1].x, pos_screen[2].x);
        let max_x = max3(pos_screen[0].x, pos_screen[1].x, pos_screen[2].x);
        let min_y = min3(pos_screen[0].y, pos_screen[1].y, pos_screen[2].y);
        let max_y = max3(pos_screen[0].y, pos_screen[1].y, pos_screen[2].y);

        let a01 = pos_screen[0].y - pos_screen[1].y;
        let b01 = pos_screen[1].x - pos_screen[0].x;
        let a12 = pos_screen[1].y - pos_screen[2].y;
        let b12 = pos_screen[2].x - pos_screen[1].x;
        let a20 = pos_screen[2].y - pos_screen[0].y;
        let b20 = pos_screen[0].x - pos_screen[2].x;

        let mut bc_screen_x_row = orient2d(pos_screen[1], pos_screen[2], min_x, min_y);
        let mut bc_screen_y_row = orient2d(pos_screen[2], pos_screen[0], min_x, min_y);
        let mut bc_screen_z_row = orient2d(pos_screen[0], pos_screen[1], min_x, min_y);

        for y in min_y..max_y {
            let mut bc_screen_x = bc_screen_x_row;
            let mut bc_screen_y = bc_screen_y_row;
            let mut bc_screen_z = bc_screen_z_row;

            for x in min_x..max_x {
                if (bc_screen_x | bc_screen_y | bc_screen_z) >= 0 {
                    if let Some(index) = self.frame_buffer.calculate_index(x, y) {
                        let mut bc_clip = Vector3::new(
                            bc_screen_x as f32 / pos_viewport[0].w,
                            bc_screen_y as f32 / pos_viewport[1].w,
                            bc_screen_z as f32 / pos_viewport[2].w,
                        );
                        bc_clip = bc_clip / (bc_clip.x + bc_clip.y + bc_clip.z);

                        let frag_depth =
                            Vector3::new(vertices[0].pos.z, vertices[1].pos.z, vertices[2].pos.z)
                                .dot(bc_clip);

                        if frag_depth < self.z_buffer[index] {
                            self.z_buffer[index] = frag_depth;

                            let color = match self.texture {
                                Some(ref texture) => {
                                    let uv = Matrix3::from_cols(
                                        vertices[0].uv.extend(0.0),
                                        vertices[1].uv.extend(0.0),
                                        vertices[2].uv.extend(0.0),
                                    ) * bc_clip;
                                    let texture_x =
                                        (uv.x * (texture.width as f32 - 1.0)).round() as usize;
                                    let texture_y =
                                        (uv.y * (texture.height as f32 - 1.0)).round() as usize;
                                    texture.colors[texture_y * texture.width + texture_x]
                                }
                                None => (Matrix3::from_cols(
                                    vertices[0].color,
                                    vertices[1].color,
                                    vertices[2].color,
                                ) * bc_clip)
                                    .map(|c| ((c * 255.0) as u8)),
                            };

                            self.frame_buffer.set_pixel_by_index(index * 4, &color);
                        }
                    }
                }

                bc_screen_x += a12;
                bc_screen_y += a20;
                bc_screen_z += a01;
            }

            bc_screen_x_row += b12;
            bc_screen_y_row += b20;
            bc_screen_z_row += b01;
        }

        self.tris_count += 1;
    }

    pub fn draw_triangle(&mut self, v0w: &Vertex, v1w: &Vertex, v2w: &Vertex) {
        let mut vs = [*v0w, *v1w, *v2w];

        for i in 0..3 {
            vs[i].pos = self.view_proj_mat * vs[i].pos;
        }

        let mut clipped_count = 0;
        let mut clipped = [Vertex::empty(); 4];

        clip_vertices(&vs[0], &vs[1], &mut clipped, &mut clipped_count);
        clip_vertices(&vs[1], &vs[2], &mut clipped, &mut clipped_count);
        clip_vertices(&vs[2], &vs[0], &mut clipped, &mut clipped_count);

        if clipped_count == 3 {
            self.rasterize_triangle(clipped[0], clipped[1], clipped[2]);
        } else if clipped_count == 4 {
            self.rasterize_triangle(clipped[0], clipped[1], clipped[3]);
            self.rasterize_triangle(clipped[1], clipped[2], clipped[3]);
        }
    }
}

pub struct FrameBuffer<'a> {
    frame: &'a mut [u8],
    width: i32,
    height: i32,
}

impl<'a> FrameBuffer<'a> {
    pub fn calculate_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            return Some((y * self.width + x) as usize);
        }
        None
    }

    pub fn set_pixel_xy(&mut self, x: i32, y: i32, c: &Vector3<u8>) {
        if let Some(index) = self.calculate_index(x, y) {
            self.set_pixel_by_index(index * 4, c);
        }
    }

    pub fn set_pixel_by_index(&mut self, index: usize, c: &Vector3<u8>) {
        self.frame[index] = c.x;
        self.frame[index + 1] = c.y;
        self.frame[index + 2] = c.z;
        self.frame[index + 3] = u8::MAX;
    }

    pub fn draw_line_2d(&mut self, p0: Vector4<f32>, p1: Vector4<f32>, c: &Vector3<u8>) {
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
                self.set_pixel_xy(y, x, c);
            } else {
                self.set_pixel_xy(x, y, c);
            }

            error += d_error;
            if error > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error -= 2 * dx;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vector4<f32>,
    pub color: Vector3<f32>,
    pub uv: Vector2<f32>,
}

impl Vertex {
    pub fn empty() -> Self {
        Self {
            pos: Vector4::zero(),
            color: Vector3::zero(),
            uv: Vector2::zero(),
        }
    }
}
