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

pub fn distance2d(x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
    (x1 - x0).hypot(y1 - y0)
}

pub fn orient2d(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>) -> f32 {
    (b.x-a.x)*(c.y-a.y) - (b.y-a.y)*(c.x-a.x)
}

pub fn barycentric(triangle: &[Vector3<f32>; 3], point: Vector2<f32>) -> Vector3<f32> {
    let mut lambda = Vector3::new(0.0, 0.0, 0.0);
    let den = 1.0
        / ((triangle[1].y - triangle[2].y) * (triangle[0].x - triangle[2].x)
            + (triangle[2].x - triangle[1].x) * (triangle[0].y - triangle[2].y));

    lambda.x = ((triangle[1].y - triangle[2].y) * (point.x - triangle[2].x)
        + (triangle[2].x - triangle[1].x) * (point.y - triangle[2].y))
        * den;
    lambda.y = ((triangle[2].y - triangle[0].y) * (point.x - triangle[2].x)
        + (triangle[0].x - triangle[2].x) * (point.y - triangle[2].y))
        * den;
    lambda.z = 1.0 - lambda.x - lambda.y;

    lambda
}
