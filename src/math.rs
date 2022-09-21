use cgmath::{Vector2, Vector3, Vector4};

#[macro_export]
macro_rules! is_between {
    ($v:expr,$min:expr,$max:expr) => {
        $v >= $min && $v <= $max
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vector4<f32>,
    pub color: Vector3<f32>,
    pub uv: Option<Vector2<f32>>,
}

pub struct VertexHelper {
    pub pos: Vector4<f32>,
    pub color: Vector3<f32>,
    pub uv: Option<Vector2<f32>>,
}

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