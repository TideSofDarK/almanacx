pub mod camera;
mod clipping;
pub mod utils;

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use cgmath::{
    InnerSpace, Matrix3, Matrix4, SquareMatrix, Vector2, Vector3, Vector4, VectorSpace, Zero,
};

use crate::{
    buffer2d::{B2D, B2DO, B2DT},
    math::{max3, min3, orient2d},
    utils::calculate_index,
};

use self::clipping::{clip_line_to_frustum, clip_triangle_to_frustum};

pub struct Renderer {
    vertex_storage: VertexStorage,
    view_proj_mat: Matrix4<f32>,
    viewport: Vector4<f32>,

    z_buffer: Vec<f32>,
    pub color_buffer: Rc<RefCell<B2DO>>,

    pub tris_count: u32,
}

impl Renderer {
    pub fn new(color_buffer: &Rc<RefCell<B2DO>>) -> Self {
        let width = color_buffer.borrow().width;
        let height = color_buffer.borrow().height;

        Self {
            vertex_storage: VertexStorage {
                vertices: Vec::with_capacity(8),
                indices: Vec::with_capacity(128),
                indices_in: Vec::with_capacity(128),
                indices_out: Vec::with_capacity(128),
            },
            view_proj_mat: Matrix4::identity(),
            viewport: Vector4::new(
                width as f32 / 2.0,
                height as f32 / 2.0,
                width as f32 / 2.0,
                height as f32 / 2.0,
            ),
            z_buffer: vec![0.0; (height * width) as usize],
            color_buffer: color_buffer.clone(),

            tris_count: 0,
        }
    }

    pub fn begin(&mut self, mat: Matrix4<f32>) {
        self.view_proj_mat = mat;
        self.color_buffer.borrow_mut().pixels.fill(20);
        self.z_buffer.fill(f32::MAX);
        self.tris_count = 0;
    }

    pub fn set_viewport(&mut self, viewport: Vector4<f32>) {
        self.viewport = Vector4::new(
            viewport.z / 2.0,
            viewport.w / 2.0,
            viewport.x + viewport.z / 2.0,
            viewport.y + viewport.w / 2.0,
        );
    }

    fn perspective_division(&self, mut pos: &mut Vector4<f32>) {
        let inv_w = 1.0 / pos.w;
        pos.x *= inv_w;
        pos.y *= inv_w;
        pos.z *= inv_w;
    }

    fn transform_viewport(&self, pos: &mut Vector4<f32>) {
        pos.x = pos.x * self.viewport.x as f32 + self.viewport.z;
        pos.y = -pos.y * self.viewport.y as f32 + self.viewport.w;
        pos.x = pos.x.round();
        pos.y = pos.y.round();

        // pos.z = 0.5 * (depth_range.sum - depth_range.diff * pos.z);
    }

    pub fn draw_gizmo(&mut self, vw: Vertex) {
        let mut v = vw;

        v.pos = self.view_proj_mat * v.pos;

        self.perspective_division(&mut v.pos);
        self.transform_viewport(&mut v.pos);

        self.color_buffer.borrow_mut().set_color(
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

        if let Some((mut p0, mut p1)) = clip_line_to_frustum(p0, p1) {
            self.perspective_division(&mut p0);
            self.perspective_division(&mut p1);

            self.transform_viewport(&mut p0);
            self.transform_viewport(&mut p1);

            self.color_buffer
                .borrow_mut()
                .draw_line_2d(p0.truncate(), p1.truncate(), &c);
        }
    }

    fn rasterize_triangle<'t, T: B2DT>(
        &mut self,
        v0: Vertex,
        v1: Vertex,
        v2: Vertex,
        texture: Option<&'t B2D<T>>,
    ) {
        let vertices: [Vertex; 3] = [v0, v1, v2];
        let mut pos_viewport = [v0.pos, v1.pos, v2.pos];
        let mut pos_screen = pos_viewport.clone();

        for i in 0..3 {
            self.transform_viewport(&mut pos_viewport[i]);

            self.perspective_division(&mut pos_screen[i]);
            self.transform_viewport(&mut pos_screen[i]);
        }

        // self.frame_buffer
        //     .draw_line_2d(pos_screen[0], pos_screen[1], Vector3::new(255, 255, 255));
        // self.frame_buffer
        //     .draw_line_2d(pos_screen[1], pos_screen[2], Vector3::new(255, 255, 255));
        // self.frame_buffer
        //     .draw_line_2d(pos_screen[2], pos_screen[0], Vector3::new(255, 255, 255));

        let pos_screen = pos_screen.map(|pos| Vector2::new(pos.x as i32, pos.y as i32));

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

        let mut color_buffer = self.color_buffer.borrow_mut();

        for y in min_y..max_y {
            let mut bc_screen_x = bc_screen_x_row;
            let mut bc_screen_y = bc_screen_y_row;
            let mut bc_screen_z = bc_screen_z_row;

            for x in min_x..max_x {
                if (bc_screen_x | bc_screen_y | bc_screen_z) >= 0 {
                    let index = calculate_index(x, y, color_buffer.width);

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

                        let color = match texture {
                            Some(ref texture) => {
                                let uv = Matrix3::from_cols(
                                    vertices[0].uv.extend(0.0),
                                    vertices[1].uv.extend(0.0),
                                    vertices[2].uv.extend(0.0),
                                ) * bc_clip;
                                texture.sample(uv.x, uv.y)
                            }
                            None => (Matrix3::from_cols(
                                vertices[0].color,
                                vertices[1].color,
                                vertices[2].color,
                            ) * bc_clip)
                                .map(|c| ((c * 255.0) as u8)),
                        };

                        color_buffer.set_color_by_index(index * 4, &color);
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

    pub fn draw_triangle<'t, T: B2DT>(
        &mut self,
        v0w: &Vertex,
        v1w: &Vertex,
        v2w: &Vertex,
        texture: Option<&'t B2D<T>>,
    ) {
        self.vertex_storage.vertices.clear();

        self.vertex_storage.vertices.push(*v0w);
        self.vertex_storage.vertices.push(*v1w);
        self.vertex_storage.vertices.push(*v2w);

        self.vertex_storage.indices.clear();

        self.vertex_storage.indices.push(0);
        self.vertex_storage.indices.push(1);
        self.vertex_storage.indices.push(2);

        for i in 0..3 {
            self.vertex_storage.vertices[i].pos =
                self.view_proj_mat * self.vertex_storage.vertices[i].pos;
        }

        if clip_triangle_to_frustum(&mut self.vertex_storage) {
            for i in 0..(self.vertex_storage.indices.len() / 3) {
                let t = i * 3;
                self.rasterize_triangle(
                    self.vertex_storage.vertices[self.vertex_storage.indices[t]],
                    self.vertex_storage.vertices[self.vertex_storage.indices[t + 1]],
                    self.vertex_storage.vertices[self.vertex_storage.indices[t + 2]],
                    texture,
                );
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

    pub fn lerp(&self, other: &Vertex, t: f32) -> Self {
        Self {
            pos: self.pos.lerp(other.pos, t),
            color: self.color.lerp(other.color, t),
            uv: self.uv.lerp(other.uv, t),
        }
    }
}

pub struct VertexStorage {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    pub indices_in: Vec<usize>,
    pub indices_out: Vec<usize>,
}
