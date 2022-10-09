use cgmath::{Matrix4, SquareMatrix, Vector2, Vector3, Vector4, Zero};

// #[derive(Copy, Clone)]
// pub struct Vector3<S> {
//     pub x: S,
//     pub y: S,
//     pub z: S
// }

// impl<S> Vector3<S> {
//     pub fn new(x: S, y: S, z: S) -> Self {
//         Self {x, y, z}
//     }
// }

// pub fn lerp3(a: Vector3<f32>, b: Vector3<f32>, t: f32) -> Vector3<f32> {
//     Vector3 { x: lerp(a.x, b.x, t), y: lerp(a.y, b.y, t), z: lerp(a.z, b.z, t) }
// }

// fn lerp(a: f32, b: f32, t: f32) -> f32 {
//     a + (b - a) * t
// }

#[inline]
pub fn min3(a: i32, b: i32, c: i32) -> i32 {
    a.min(b.min(c))
}

#[inline]
pub fn max3(a: i32, b: i32, c: i32) -> i32 {
    a.max(b.max(c))
}

#[inline]
pub fn orient2d(a: Vector2<i32>, b: Vector2<i32>, x: i32, y: i32) -> i32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let mut m = Matrix4::zero();

    let tan_half_fov_inverse = 1.0 / (fov * 0.5).tan();

    m[0][0] = tan_half_fov_inverse / aspect;
    m[1][1] = tan_half_fov_inverse;
    m[2][2] = -far / (far - near);
    m[3][2] = -(far * near) / (far - near);
    m[2][3] = -1.0;

    m
}
