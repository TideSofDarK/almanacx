pub mod camera;
mod clipping;
mod context3d;
mod texture;
pub mod utils;

use cgmath::{Matrix4, Vector2, Vector3, Vector4, VectorSpace, Zero};

use crate::draw_target::DrawTarget;

use self::context3d::RenderContext3D;

pub struct Renderer {
    z_buffer: Vec<f32>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            z_buffer: vec![0.0; height * width],
        }
    }

    pub fn begin(&mut self) {
        self.z_buffer.fill(f32::MAX);
    }

    pub fn create_context_3d<'c, 'z>(
        &'z mut self,
        view_proj_mat: Matrix4<f32>,
        draw_target: &'c mut DrawTarget<'c>,
    ) -> RenderContext3D<'c, 'z> {
        RenderContext3D::new(view_proj_mat, draw_target, self.z_buffer.as_mut_slice())
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
