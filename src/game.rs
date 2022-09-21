use std::time::Instant;

use cgmath::{Vector2, Vector3, Vector4};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{
    math::Vertex,
    player::Player,
    renderer::Renderer,
    wad::{self},
    world::World,
    HEIGHT, WIDTH,
};

enum GameState {
    Action,
    Automap,
}

pub struct Game {
    game_state: GameState,
    renderer: Renderer,
    player: Player,
    world: World,
    triangles: Vec<(Vertex, Vertex, Vertex)>,
}

impl Game {
    pub fn new() -> Self {
        let wad = wad::load("./assets/DOOM1.WAD").expect("no such wad file");
        let map_data = wad.get_map_data("E1M1").expect("no such map");

        let mut renderer = Renderer::new(WIDTH, HEIGHT);
        renderer.set_texture(wad.get_texture_data("WALL03_7"));

        let mut triangles: Vec<(Vertex, Vertex, Vertex)> = vec![];
        for x in 0..2 {
            for z in 0..2 {
                let offset_x = 0.5 * x as f32 + 3.0;
                let offset_z = 0.5 * z as f32 + 3.0;
                let offset_y = 0.0 + 0.0;
                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, 0.25 + offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Some(Vector2::new(1.0, 1.0)),
                };
                let v1 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, -0.25 + offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Some(Vector2::new(1.0, 0.0)),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, -0.25 + offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Some(Vector2::new(0.0, 0.0)),
                };

                triangles.push((v0, v1, v2));

                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, offset_z + 0.25, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Some(Vector2::new(1.0, 1.0)),
                };
                let v1 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, offset_z + 0.25, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Some(Vector2::new(0.0, 1.0)),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, offset_z - 0.25, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Some(Vector2::new(0.0, 0.0)),
                };

                triangles.push((v2, v1, v0));
            }
        }

        for x in 0..2 {
            for z in 0..2 {
                let offset_x = 0.5 * x as f32;
                let offset_y = 0.5 * z as f32;
                let offset_z = ((2 * x) + z) as f32;
                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, 0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Some(Vector2::new(1.0, 1.0)),
                };
                let v1 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, -0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Some(Vector2::new(1.0, 0.0)),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, -0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Some(Vector2::new(0.0, 0.0)),
                };

                triangles.push((v0, v1, v2));

                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y + 0.25, offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Some(Vector2::new(1.0, 1.0)),
                };
                let v1 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y + 0.25, offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Some(Vector2::new(0.0, 1.0)),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y - 0.25, offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Some(Vector2::new(0.0, 0.0)),
                };

                triangles.push((v2, v1, v0));
            }
        }

        Self {
            game_state: GameState::Action,
            renderer: renderer,
            player: Player::new(),
            world: World::new(map_data),
            triangles,
        }
    }

    pub fn handle_input(&mut self, input: &WinitInputHelper) -> bool {
        if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
            return true;
        }

        self.player.handle_input(
            input.key_held(VirtualKeyCode::W),
            input.key_held(VirtualKeyCode::S),
            input.key_held(VirtualKeyCode::A),
            input.key_held(VirtualKeyCode::D),
            input.key_held(VirtualKeyCode::Left),
            input.key_held(VirtualKeyCode::Right),
            input.key_held(VirtualKeyCode::LShift),
        );

        if input.key_pressed(VirtualKeyCode::Tab) {
            self.game_state = match self.game_state {
                GameState::Action => GameState::Automap,
                _ => GameState::Action,
            }
        }

        // if input.key_pressed(VirtualKeyCode::Key1) {
        //     self.renderer.set_texture(*self.texture);
        // }

        if input.key_pressed(VirtualKeyCode::Key1) {
            // self.renderer.set_texture(*self.texture);
        }

        false
    }

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt);
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        self.renderer.begin(self.player.get_view_matrix());

        match self.game_state {
            GameState::Action => {
                for (v0, v1, v2) in &self.triangles {
                    self.renderer.draw_triangle(v0, v1, v2, frame);
                }
            }
            GameState::Automap => {
                let vertices = self.world.get_vertices();

                for v in self.world.get_linedefs() {
                    self.renderer.draw_line(vertices[v.x], vertices[v.y], frame);
                }
            }
            _ => {}
        }
    }
}
