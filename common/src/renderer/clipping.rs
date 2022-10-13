use std::mem;

use cgmath::{InnerSpace, Vector4, VectorSpace};

use super::Vertex;

pub const CLIPPING_PLANE: f32 = 0.00001;

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

pub fn clip_line_to_frustum(p0: &mut Vector4<f32>, p1: &mut Vector4<f32>) -> bool {
    let mask0 = count_frustum_clip_mask(p0);
    let mask1 = count_frustum_clip_mask(p1);

    let mut t0: f32 = 0.0;
    let mut t1: f32 = 1.0;

    let mask = mask0 | mask1;
    if mask != 0 {
        for plane_index in 0..6 {
            if (mask & FRUSTUM_CLIP_MASK[plane_index]) != 0 {
                let d0 = FRUSTUM_CLIP_PLANE[plane_index].dot(*p0);
                let d1 = FRUSTUM_CLIP_PLANE[plane_index].dot(*p1);

                if d0 < 0.0 && d1 < 0.0 {
                    return true;
                } else if d0 < 0.0 {
                    let t = -d0 / (d1 - d0);
                    t0 = t0.max(t);
                } else {
                    let t = d0 / (d0 - d1);
                    t1 = t1.min(t);
                }
            }
        }
    }

    let p0_original = *p0;
    let p1_original = *p1;

    if mask0 != 0 {
        *p0 = p0_original.lerp(p1_original, t0);
    }

    if mask1 != 0 {
        *p1 = p0_original.lerp(p1_original, t1);
    }

    return false;
}

pub fn clip_triangle_to_frustum(vs: &mut Vec<Vertex>) -> Option<Vec<usize>> {
    let mut indices: Vec<usize> = vec![0, 1, 2];

    let mask = count_frustum_clip_mask(&vs[0].pos)
        | count_frustum_clip_mask(&vs[1].pos)
        | count_frustum_clip_mask(&vs[2].pos);

    if mask == 0 {
        return Some(indices);
    }

    let mut indices_in: Vec<usize> = vec![0, 1, 2];
    let mut indices_out: Vec<usize> = vec![];

    let mut full_clip = false;

    for plane_index in 0..6 {
        if (mask & FRUSTUM_CLIP_MASK[plane_index]) != 0 {
            if indices_in.len() < 3 {
                full_clip = true;
                break;
            }

            indices_out.clear();

            let mut idx_pre = indices_in[0];
            let mut d_pre = FRUSTUM_CLIP_PLANE[plane_index].dot(vs[idx_pre].pos);

            indices_in.push(idx_pre);

            for i in 1..indices_in.len() {
                let idx = indices_in[i];
                let d = FRUSTUM_CLIP_PLANE[plane_index].dot(vs[idx].pos);

                if d_pre >= 0.0 {
                    indices_out.push(idx_pre);
                }

                if d_pre.is_sign_negative() ^ d.is_sign_negative() {
                    let t = if d < 0.0 {
                        d_pre / (d_pre - d)
                    } else {
                        -d_pre / (d - d_pre)
                    };
                    let vertex = vs[idx_pre].lerp(&vs[idx], t);

                    vs.push(vertex);
                    indices_out.push((vs.len() - 1) as usize);
                }

                idx_pre = idx;
                d_pre = d;
            }

            mem::swap(&mut indices_in, &mut indices_out);
        }
    }

    if (full_clip || indices_in.is_empty()) {
        return None;
    }

    indices[0] = indices_in[0];
    indices[1] = indices_in[1];
    indices[2] = indices_in[2];

    for i in 3..indices_in.len() {
        indices.push(indices_in[0]);
        indices.push(indices_in[i - 1]);
        indices.push(indices_in[i]);
    }

    Some(indices)
}
