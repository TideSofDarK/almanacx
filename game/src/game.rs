use cgmath::{Vector2, Vector3, Vector4, Zero};
use common::{
    application::Application,
    draw_target::DrawTarget,
    renderer::{camera::Camera, utils::draw_grid, Renderer, Vertex},
    virtual_window::VirtualWindow,
    wad::{self, TextureData},
};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{player::Player, world::World};

const PRIMARY_WIDTH: usize = 640;
const PRIMARY_HEIGHT: usize = 480;

const REFERENCE_WIDTH: usize = 960;
const REFERENCE_HEIGHT: usize = 540;

enum GameState {
    Action,
    Automap,
}

pub struct Game {
    primary_window: VirtualWindow,
    game_state: GameState,
    renderer: Renderer,
    camera: Camera,
    player: Player,
    world: World,
    triangles: Vec<(Vertex, Vertex, Vertex)>,
    texture: Option<TextureData>,
}

impl Game {
    pub fn new() -> Self {
        let wad = wad::load("./assets/DOOM1.WAD").expect("no such wad file");
        let map_data = wad.get_map_data("E1M1").expect("no such map");

        let renderer = Renderer::new(PRIMARY_WIDTH, PRIMARY_HEIGHT);
        let mut camera = Camera::new();
        camera.set_perspective(
            f32::to_radians(60.0),
            PRIMARY_WIDTH as f32 / PRIMARY_HEIGHT as f32,
            0.1,
            100.0,
        );

        let mut triangles: Vec<(Vertex, Vertex, Vertex)> = vec![];
        for x in 0..5 {
            for z in 0..5 {
                let offset_x = 0.5 * x as f32 + 1.25;
                let offset_z = 0.5 * z as f32 + 1.25;
                let offset_y = 0.0;
                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, 0.25 + offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Vector2::new(1.0, 1.0),
                };
                let v1 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, -0.25 + offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Vector2::new(1.0, 0.0),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, -0.25 + offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Vector2::new(0.0, 0.0),
                };

                triangles.push((v2, v1, v0));

                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y, offset_z + 0.25, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Vector2::new(1.0, 1.0),
                };
                let v1 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, offset_z + 0.25, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Vector2::new(0.0, 1.0),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y, offset_z - 0.25, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Vector2::new(0.0, 0.0),
                };

                triangles.push((v0, v1, v2));
            }
        }

        for x in 0..5 {
            for z in 0..5 {
                let offset_x = 0.5 * x as f32;
                let offset_y = 0.5 * z as f32;
                let offset_z = ((2 * x) + z) as f32;
                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, 0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Vector2::new(1.0, 1.0),
                };
                let v1 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, -0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Vector2::new(1.0, 0.0),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, -0.25 + offset_y, offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Vector2::new(0.0, 0.0),
                };

                triangles.push((v0, v1, v2));

                let v0 = Vertex {
                    pos: Vector4::new(-0.25 + offset_x, offset_y + 0.25, offset_z, 1.0),
                    color: Vector3::new(1.0, 0.0, 0.0),
                    uv: Vector2::new(1.0, 1.0),
                };
                let v1 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y + 0.25, offset_z, 1.0),
                    color: Vector3::new(0.0, 1.0, 0.0),
                    uv: Vector2::new(0.0, 1.0),
                };
                let v2 = Vertex {
                    pos: Vector4::new(0.25 + offset_x, offset_y - 0.25, offset_z, 1.0),
                    color: Vector3::new(0.0, 0.0, 1.0),
                    uv: Vector2::new(0.0, 0.0),
                };

                triangles.push((v2, v1, v0));
            }
        }

        let primary_window_x = (REFERENCE_WIDTH - PRIMARY_WIDTH) / 2;
        let primary_window_y = (REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2;

        Self {
            primary_window: VirtualWindow::new(
                primary_window_x,
                primary_window_y,
                PRIMARY_WIDTH,
                PRIMARY_HEIGHT,
            ),
            game_state: GameState::Action,
            renderer: renderer,
            camera: camera,
            player: Player::new(),
            world: World::new(map_data),
            triangles: triangles,
            texture: Some(wad.get_texture_data("WALL03_7")),
        }
    }
}

impl Application for Game {
    fn get_name(&self) -> &'static str {
        "Almanac X"
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        self.player.handle_input(
            input.key_held(VirtualKeyCode::W),
            input.key_held(VirtualKeyCode::S),
            input.key_held(VirtualKeyCode::A),
            input.key_held(VirtualKeyCode::D),
            input.key_held(VirtualKeyCode::Left),
            input.key_held(VirtualKeyCode::Right),
            input.key_held(VirtualKeyCode::LShift),
        );

        // if input.key_pressed(VirtualKeyCode::F11) {
        //     println!("{:?}", self.renderer.get_tris_count());
        // }

        if input.key_pressed(VirtualKeyCode::Tab) {
            self.game_state = match self.game_state {
                GameState::Action => GameState::Automap,
                _ => GameState::Action,
            }
        }

        // if input.key_pressed(VirtualKeyCode::Z) {
        //     self.renderer.set_texture(self.texture.take());
        // }

        // if input.key_pressed(VirtualKeyCode::X) {
        //     self.texture = self.renderer.take_texture();
        // }
    }

    fn update(&mut self, dt: f32) {
        self.player.update(dt);
    }

    fn draw(&mut self, target: &mut DrawTarget<'_>) {
        self.renderer.begin();

        let mut primary_window_target = self.primary_window.get_draw_target();
        primary_window_target.clear();
        let mut ctx = self.renderer.create_context_3d(
            self.camera.get_projection() * self.player.get_view(),
            &mut primary_window_target,
        );

        draw_grid(&mut ctx, Vector3::<f32>::zero(), 0.5);

        ctx.draw_line(
            Vector4 {
                x: -8.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            Vector4 {
                x: 8.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            Vector3::new(255, 255, 255),
        );

        match self.game_state {
            GameState::Action => {
                for (v0, v1, v2) in &self.triangles {
                    ctx.draw_triangle(v0, v1, v2);
                }
            }
            GameState::Automap => {
                let vertices = self.world.get_vertices();

                for v in self.world.get_linedefs() {
                    ctx.draw_line(
                        vertices[v.x].pos,
                        vertices[v.y].pos,
                        Vector3::new(255, 255, 255),
                    );

                    // ctx.draw_gizmo(vertices[v.x]);
                    // ctx.draw_gizmo(vertices[v.y]);
                }
            }
        }

        self.primary_window.draw(target);
    }

    fn resize_window(&mut self, width: u32, height: u32) {}

    fn get_reference_dimensions(&self) -> Option<(u32, u32)> {
        Some((REFERENCE_WIDTH as u32, REFERENCE_HEIGHT as u32))
    }
}
