use cgmath::{Vector2, Vector3, Vector4, Zero};
use common::{renderer::Vertex, wad::WorldData};

const MAP_SCALE: f32 = 0.01;

pub struct World {
    vertices: Vec<Vertex>,
    linedefs: Vec<Vector2<usize>>,
}

impl World {
    pub fn new(data: WorldData) -> Self {
        let mut vertices: Vec<Vertex> = vec![];
        for raw_v in data.vertices {
            vertices.push(Vertex {
                pos: Vector4::new(
                    raw_v.x as f32 * MAP_SCALE,
                    -0.5,
                    raw_v.y as f32 * MAP_SCALE,
                    1.0,
                ),
                color: Vector3::new(1.0, 1.0, 1.0),
                uv: Vector2::zero(),
            });
        }

        // for v in &vertices {
        //     println!("Vertex X:{} Y:{}", v.pos.x, v.pos.z);
        // }

        Self {
            vertices: vertices,
            linedefs: data.linedefs,
        }
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_linedefs(&self) -> &Vec<Vector2<usize>> {
        &self.linedefs
    }
}
