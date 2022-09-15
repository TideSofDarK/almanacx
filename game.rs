use std::time::Instant;

use cgmath::{Vector3, Vector4};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{math::Vertex, player::Player, renderer::Renderer, wad, world::World, HEIGHT, WIDTH};

enum GameState {
    Action,
    Automap,
}

pub struct Game {
    game_state: GameState,
    last_frame: Instant,
    renderer: Renderer,
    player: Player,
    map: World,
    dt: f32,
}

impl Game {
    pub fn new() -> Self {
        let wad = wad::load("./DOOM1.WAD").expect("no such wad file");
        //let dir_offset = wad.read_4_bytes(8) as usize;
        // println!("{:?}", wad.read_u32(4) as usize);
        // println!("{:?}", wad.read_str_4bytes(0));
        let map_data = wad.get_map_data("E1M1").expect("no such map");

        Self {
            game_state: GameState::Action,
            last_frame: Instant::now(),
            renderer: Renderer::new(WIDTH, HEIGHT),
            player: Player::new(),
            map: World::new(map_data),
            dt: 0.0,
        }
    }

    pub fn dt(&mut self) {
        self.dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
    }

    pub fn handle_input(&mut self, input: &mut WinitInputHelper) {
        self.player.handle_input(
            input.key_held(VirtualKeyCode::W),
            input.key_held(VirtualKeyCode::S),
            input.key_held(VirtualKeyCode::A),
            input.key_held(VirtualKeyCode::D),
            input.key_held(VirtualKeyCode::Left),
            input.key_held(VirtualKeyCode::Right),
            input.key_held(VirtualKeyCode::LShift),
            self.dt,
        );

        if input.key_pressed(VirtualKeyCode::Tab) {
            self.game_state = match self.game_state {
                GameState::Action => GameState::Automap,
                _ => GameState::Action,
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn draw(&mut self, frame: &mut [u8]) {
        for pixel in frame.iter_mut() {
            *pixel = 0x00;
        }

        self.renderer.begin(self.player.get_view_matrix());

        match self.game_state {
            GameState::Action => {
                for i in 0..15000 {
                    let offset_z = (i as f32) * 0.5;
                    let offset_y = (i as f32) * 0.0;
                    let v0 = Vertex {
                        pos: Vector4::new(-0.25, 0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(1.0, 0.0, 0.0),
                    };
                    let v1 = Vertex {
                        pos: Vector4::new(-0.25, -0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(0.0, 1.0, 0.0),
                    };
                    let v2 = Vertex {
                        pos: Vector4::new(0.25, -0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(0.0, 0.0, 1.0),
                    };

                    self.renderer.draw_triangle(v0, v1, v2, frame);

                    let v0 = Vertex {
                        pos: Vector4::new(-0.25, 0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(1.0, 0.0, 0.0),
                    };
                    let v1 = Vertex {
                        pos: Vector4::new(0.25, 0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(0.0, 1.0, 0.0),
                    };
                    let v2 = Vertex {
                        pos: Vector4::new(0.25, -0.25 + offset_y, 0.0 + offset_z, 1.0),
                        color: Vector3::new(0.0, 0.0, 1.0),
                    };

                    self.renderer.draw_triangle(v0, v1, v2, frame);
                }
            }
            GameState::Automap => {
                let vertices = self.map.get_vertices();

                for v in self.map.get_linedefs() {
                    self.renderer.draw_line(vertices[v.x], vertices[v.y], frame);
                }
            }
            _ => {}
        }
    }
}
