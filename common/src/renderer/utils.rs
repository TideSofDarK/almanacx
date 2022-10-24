use cgmath::Vector3;

use crate::{buffer2d::B2DT, utils::color_from_tuple};

use super::Renderer;

const GRID_SIZE: i32 = 12;
const GRID_COLOR: u16 = color_from_tuple((31, 31, 31));

pub fn draw_grid(renderer: &mut Renderer, origin_reference: Vector3<f32>, cell_size: f32) {
    for i in -GRID_SIZE..GRID_SIZE {
        renderer.draw_line(
            Vector3::new(i as f32 * cell_size, 0.0, -GRID_SIZE as f32 * cell_size).extend(1.0),
            Vector3::new(
                i as f32 * cell_size,
                0.0,
                (GRID_SIZE - 1) as f32 * cell_size,
            )
            .extend(1.0),
            GRID_COLOR,
        );

        renderer.draw_line(
            Vector3::new(-GRID_SIZE as f32 * cell_size, 0.0, i as f32 * cell_size).extend(1.0),
            Vector3::new(
                (GRID_SIZE - 1) as f32 * cell_size,
                0.0,
                i as f32 * cell_size,
            )
            .extend(1.0),
            GRID_COLOR,
        );
    }
}
