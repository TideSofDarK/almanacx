use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3, Zero};

use crate::math::perspective;

pub struct Camera {
    aspect: f32,
    fov: f32,
    near: f32,
    far: f32,
    proj: Matrix4<f32>,
    view: Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect: 0.0,
            fov: 0.0,
            near: 0.0,
            far: 0.0,
            proj: Matrix4::zero(),
            view: Matrix4::zero(),
        }
    }

    pub fn set_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        self.proj = perspective(fov, aspect, near, far);
    }

    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.proj
    }

    // pub fn get_view(
    //     &self,
    //     eye: Vector3<f32>,
    //     center: Vector3<f32>,
    //     up: Vector3<f32>,
    // ) -> Matrix4<f32> {

    //     look_at_rh(eye, center, up)
    // }
}
