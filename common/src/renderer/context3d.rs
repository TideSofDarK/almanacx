use cgmath::{InnerSpace, Matrix3, Matrix4, Vector2, Vector3, Vector4};

use crate::{
    draw_target::DrawTarget,
    math::{max3, min3, orient2d},
    wad::TextureData,
};

use super::{
    clipping::{clip_line_to_frustum, clip_triangle_to_frustum},
    Vertex,
};

pub struct RenderContext3D<'d, 'r> {
    view_proj_mat: Matrix4<f32>,
    draw_target: &'d mut DrawTarget<'d>,
    z_buffer: &'r mut [f32],
    tris_count: u32,
    viewport: Vector4<f32>,
    texture: Option<TextureData>,
}

impl<'d, 'r> RenderContext3D<'d, 'r> {
    pub fn new(
        view_proj_mat: Matrix4<f32>,
        draw_target: &'d mut DrawTarget<'d>,
        z_buffer: &'r mut [f32],
    ) -> Self {
        let half_width = draw_target.get_width_f() / 2.0;
        let half_height = draw_target.get_height_f() / 2.0;
        let viewport = Vector4::new(half_width, half_height, half_width, half_height);

        Self {
            view_proj_mat: view_proj_mat,
            draw_target: draw_target,
            z_buffer: z_buffer,
            tris_count: 0,
            viewport: viewport,
            texture: None,
        }
    }

    pub fn with_viewport(mut self, viewport: Vector4<f32>) -> Self {
        self.viewport = Vector4::new(
            viewport.z / 2.0,
            viewport.w / 2.0,
            viewport.x + viewport.z / 2.0,
            viewport.y + viewport.w / 2.0,
        );

        self
    }

    #[inline]
    pub fn get_tris_count(&self) -> u32 {
        self.tris_count
    }

    #[inline]
    fn perspective_division(&self, mut pos: &mut Vector4<f32>) {
        let inv_w = 1.0 / pos.w;
        pos.x *= inv_w;
        pos.y *= inv_w;
        pos.z *= inv_w;
    }

    #[inline]
    fn transform_viewport(&self, pos: &mut Vector4<f32>) {
        pos.x = pos.x * self.viewport.x as f32 + self.viewport.z;
        pos.y = pos.y * self.viewport.y as f32 + self.viewport.w;
        pos.x = pos.x.round();
        pos.y = pos.y.round();

        // pos.z = 0.5 * (depth_range.sum - depth_range.diff * pos.z);
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

        self.perspective_division(&mut v.pos);
        self.transform_viewport(&mut v.pos);

        self.draw_target.set_color_xy(
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

            self.draw_target
                .draw_line_2d(p0.truncate(), p1.truncate(), &c);
        }
    }

    fn rasterize_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex) {
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

        for y in min_y..max_y {
            let mut bc_screen_x = bc_screen_x_row;
            let mut bc_screen_y = bc_screen_y_row;
            let mut bc_screen_z = bc_screen_z_row;

            for x in min_x..max_x {
                if (bc_screen_x | bc_screen_y | bc_screen_z) >= 0 {
                    if let Some(index) = self.draw_target.calculate_index(x, y) {
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

                            self.draw_target.set_color_by_index(index * 4, &color);
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
        let mut vs = vec![*v0w, *v1w, *v2w];

        for i in 0..3 {
            vs[i].pos = self.view_proj_mat * vs[i].pos;
        }

        if let Some(indices) = clip_triangle_to_frustum(&mut vs) {
            for triangle in indices.chunks(3) {
                self.rasterize_triangle(vs[triangle[0]], vs[triangle[1]], vs[triangle[2]]);
            }
        }
    }
}
