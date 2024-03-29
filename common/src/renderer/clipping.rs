use std::mem;

use cgmath::{InnerSpace, Vector4, VectorSpace};

use super::VertexStorage;

pub const FRUSTUM_CLIP_MASK: [u8; 6] = [1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5];

pub const FRUSTUM_CLIP_PLANE: [Vector4<f32>; 6] = [
    Vector4::new(-1.0, 0.0, 0.0, 1.0),
    Vector4::new(1.0, 0.0, 0.0, 1.0),
    Vector4::new(0.0, -1.0, 0.0, 1.0),
    Vector4::new(0.0, 1.0, 0.0, 1.0),
    Vector4::new(0.0, 0.0, -1.0, 1.0),
    Vector4::new(0.0, 0.0, 1.0, 1.0),
];

pub fn count_frustum_clip_mask(clip_pos: &Vector4<f32>) -> u8 {
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

pub fn clip_line_to_frustum(
    mut p0: Vector4<f32>,
    mut p1: Vector4<f32>,
) -> Option<(Vector4<f32>, Vector4<f32>)> {
    let mask0 = count_frustum_clip_mask(&p0);
    let mask1 = count_frustum_clip_mask(&p1);

    if (mask0 & mask1) != 0 {
        return None;
    }

    let mut t0: f32 = 0.0;
    let mut t1: f32 = 1.0;

    let mask = mask0 | mask1;
    if mask == 0 {
        return Some((p0, p1));
    } else {
        // Dot product calculation optimization
        // Could be:    let bc0 = FRUSTUM_CLIP_PLANE[plane_index].dot(*p0);
        //              let bc1 = FRUSTUM_CLIP_PLANE[plane_index].dot(*p1);
        // inside for loop
        let bc0 = [
            p0.w - p0.x,
            p0.w + p0.x,
            p0.w - p0.y,
            p0.w + p0.y,
            p0.w - p0.z,
            p0.w + p0.z,
        ];
        let bc1 = [
            p1.w - p1.x,
            p1.w + p1.x,
            p1.w - p1.y,
            p1.w + p1.y,
            p1.w - p1.z,
            p1.w + p1.z,
        ];

        for plane_index in 0..6 {
            if bc1[plane_index] < 0.0 {
                let t = bc0[plane_index] / (bc0[plane_index] - bc1[plane_index]);
                t1 = t1.min(t);
            } else if bc0[plane_index] < 0.0 {
                let t = bc0[plane_index] / (bc0[plane_index] - bc1[plane_index]);
                t0 = t0.max(t);
            }
            if t0 > t1 {
                return None;
            }
        }
    }

    let mut temp = p0;

    if mask0 != 0 {
        temp = p0.lerp(p1, t0);
    }

    if mask1 != 0 {
        p1 = p0.lerp(p1, t1);
    }

    p0 = temp;

    return Some((p0, p1));
}

pub fn clip_triangle_to_frustum(vertex_storage: &mut VertexStorage) -> bool {
    let mask = count_frustum_clip_mask(&vertex_storage.vertices[0].pos)
        | count_frustum_clip_mask(&vertex_storage.vertices[1].pos)
        | count_frustum_clip_mask(&vertex_storage.vertices[2].pos);

    if mask == 0 {
        return true;
    }

    vertex_storage.indices_in.clear();
    vertex_storage.indices_in.push(0);
    vertex_storage.indices_in.push(1);
    vertex_storage.indices_in.push(2);

    vertex_storage.indices_out.clear();

    let mut full_clip = false;

    for plane_index in 0..6 {
        if (mask & FRUSTUM_CLIP_MASK[plane_index]) != 0 {
            if vertex_storage.indices_in.len() < 3 {
                full_clip = true;
                break;
            }

            vertex_storage.indices_out.clear();

            let mut idx_pre = vertex_storage.indices_in[0];
            let mut d_pre =
                FRUSTUM_CLIP_PLANE[plane_index].dot(vertex_storage.vertices[idx_pre].pos);

            vertex_storage.indices_in.push(idx_pre);

            for i in 1..vertex_storage.indices_in.len() {
                let idx = vertex_storage.indices_in[i];
                let d = FRUSTUM_CLIP_PLANE[plane_index].dot(vertex_storage.vertices[idx].pos);

                if d_pre >= 0.0 {
                    vertex_storage.indices_out.push(idx_pre);
                }

                if d_pre.is_sign_negative() ^ d.is_sign_negative() {
                    let t = if d < 0.0 {
                        d_pre / (d_pre - d)
                    } else {
                        -d_pre / (d - d_pre)
                    };
                    let vertex =
                        vertex_storage.vertices[idx_pre].lerp(&vertex_storage.vertices[idx], t);

                    vertex_storage.vertices.push(vertex);
                    vertex_storage
                        .indices_out
                        .push((vertex_storage.vertices.len() - 1) as usize);
                }

                idx_pre = idx;
                d_pre = d;
            }

            mem::swap(
                &mut vertex_storage.indices_in,
                &mut vertex_storage.indices_out,
            );
        }
    }

    if full_clip || vertex_storage.indices_in.is_empty() {
        return false;
    }

    vertex_storage.indices[0] = vertex_storage.indices_in[0];
    vertex_storage.indices[1] = vertex_storage.indices_in[1];
    vertex_storage.indices[2] = vertex_storage.indices_in[2];

    for i in 3..vertex_storage.indices_in.len() {
        vertex_storage.indices.push(vertex_storage.indices_in[0]);
        vertex_storage
            .indices
            .push(vertex_storage.indices_in[i - 1]);
        vertex_storage.indices.push(vertex_storage.indices_in[i]);
    }

    true
}
