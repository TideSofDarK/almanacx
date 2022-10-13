use cgmath::Vector3;

use super::RenderContext3D;

const GRID_SIZE: i32 = 12;
const GRID_COLOR: Vector3<u8> = Vector3::new(255, 255, 255);

pub fn draw_grid(ctx: &mut RenderContext3D, origin_reference: Vector3<f32>, cell_size: f32) {
    for i in -GRID_SIZE..GRID_SIZE {
        ctx.draw_line(
            Vector3::new(i as f32 * cell_size, 0.0, -GRID_SIZE as f32 * cell_size).extend(1.0),
            Vector3::new(
                i as f32 * cell_size,
                0.0,
                (GRID_SIZE - 1) as f32 * cell_size,
            )
            .extend(1.0),
            GRID_COLOR,
        );

        ctx.draw_line(
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
