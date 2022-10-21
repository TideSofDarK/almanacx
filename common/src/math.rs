use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector2, Vector3, Vector4, Zero};

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

pub fn min3(a: i32, b: i32, c: i32) -> i32 {
    a.min(b.min(c))
}

pub fn max3(a: i32, b: i32, c: i32) -> i32 {
    a.max(b.max(c))
}

pub fn orient2d(a: Vector2<i32>, b: Vector2<i32>, x: i32, y: i32) -> i32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let ymax = near * (fov / 2.0).tan();
    let xmax = ymax * aspect;

    let c0r0 = (2.0 * near) / (xmax - -xmax);
    let c0r1 = 0.0;
    let c0r2 = 0.0;
    let c0r3 = 0.0;

    let c1r0 = 0.0;
    let c1r1 = (2.0 * near) / (ymax - -ymax);
    let c1r2 = 0.0;
    let c1r3 = 0.0;

    let c2r0 = (xmax + -xmax) / (xmax - -xmax);
    let c2r1 = (ymax + -ymax) / (ymax - -ymax);
    let c2r2 = -(far + near) / (far - near);
    let c2r3 = -1.0;

    let c3r0 = 0.0;
    let c3r1 = 0.0;
    let c3r2 = -(2.0 * far * near) / (far - near);
    let c3r3 = 0.0;

    Matrix4::new(
        c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1, c3r2,
        c3r3,
    )
}

pub fn look_at(eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let forward = (center - eye).normalize();
    let side = forward.cross(up).normalize();
    // let up = side.cross(forward);

    let mut view = Matrix4::identity();

    view[0][0] = side.x;
    view[1][0] = side.y;
    view[2][0] = side.z;
    view[3][0] = -side.dot(eye);

    view[0][1] = up.x;
    view[1][1] = up.y;
    view[2][1] = up.z;
    view[3][1] = -up.dot(eye);

    view[0][2] = -forward.x;
    view[1][2] = -forward.y;
    view[2][2] = -forward.z;
    view[3][2] = forward.dot(eye);

    view
}
