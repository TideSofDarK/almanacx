use cgmath::{InnerSpace, Matrix4, Rad, SquareMatrix, Vector3, Zero};

use crate::math::{look_at, perspective};

pub struct Camera {
    near: f32,
    far: f32,
    pub proj: Matrix4<f32>,
    view: Matrix4<f32>,
}

impl Camera {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            near: near,
            far: far,
            proj: perspective(fov, aspect, near, far),
            view: Matrix4::identity(),
        }
    }

    pub fn get_view(
        &self,
        eye: Vector3<f32>,
        center: Vector3<f32>,
        up: Vector3<f32>,
    ) -> Matrix4<f32> {
        look_at(eye, center, up)
    }
}
