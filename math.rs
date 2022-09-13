use cgmath::{Vector3, Vector4};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vector4<f32>,
    pub color: Vector3<f32>
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

pub fn distance2D(x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
	return (x1 - x0).hypot(y1 - y0)
}