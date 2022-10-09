use cgmath::Vector4;

use super::Vertex;

pub const FRUSTUM_CLIP_MASK: [i32; 6] = [1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5];

pub const FRUSTUM_CLIP_PLANE: [Vector4<f32>; 6] = [
    Vector4::new(-1.0, 0.0, 0.0, 1.0),
    Vector4::new(1.0, 0.0, 0.0, 1.0),
    Vector4::new(0.0, -1.0, 0.0, 1.0),
    Vector4::new(0.0, 1.0, 0.0, 1.0),
    Vector4::new(0.0, 0.0, -1.0, 1.0),
    Vector4::new(0.0, 0.0, 1.0, 1.0),
];

pub fn count_frustum_clip_mask(clip_pos: Vector4<f32>) -> i32 {
    let mut mask = 0;
    if clip_pos.w < clip_pos.x {
        mask |= FRUSTUM_CLIP_MASK[0];
    }
    if clip_pos.w < -clip_pos.x {
        mask |= FRUSTUM_CLIP_MASK[1];
    }
    if clip_pos.w < clip_pos.y {
        mask |= FRUSTUM_CLIP_MASK[2];
    }
    if clip_pos.w < -clip_pos.y {
        mask |= FRUSTUM_CLIP_MASK[3];
    }
    if clip_pos.w < clip_pos.z {
        mask |= FRUSTUM_CLIP_MASK[4];
    }
    if clip_pos.w < -clip_pos.z {
        mask |= FRUSTUM_CLIP_MASK[5];
    }
    return mask;
}

pub fn clip_vertices(
    v0: &Vertex,
    v1: &Vertex,
    clipped: &mut [Vertex; 4],
    clipped_count: &mut usize,
) {
    let ok0 = v0.pos.z > 0.0;
    let ok1 = v1.pos.z > 0.0;

    if ok0 & ok1 {
        clipped[*clipped_count] = *v0;
        *clipped_count += 1;
    } else if ok0 ^ ok1 {
        if ok0 {
            clipped[*clipped_count] = *v0;
            *clipped_count += 1;
        }

        let diff = v1.pos - v0.pos;
        let t = -v0.pos.z / diff.z;

        let ref mut vertex = clipped[*clipped_count];
        *clipped_count += 1;

        vertex.pos = v0.pos + diff * t;
        vertex.uv = v0.uv + (v1.uv - v0.uv) * t;
        vertex.color = v0.color + (v1.color - v0.color) * t;
    }
}
