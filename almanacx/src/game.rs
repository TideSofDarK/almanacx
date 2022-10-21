use cgmath::{Vector2, Vector3, Vector4, Zero};
use common::{
    buffer2d::{Buffer2D, Buffer2DSlice},
    image::bmp,
    platform::{
        input::{Input, INPUT_LMB},
        Application,
    },
    renderer::{camera::Camera, utils::draw_grid, Renderer, Vertex},
    virtual_window::VirtualWindow,
    vk,
    wad::{self},
};

use crate::{player::Player, world::World};

const PRIMARY_WIDTH: i32 = 580;
const PRIMARY_HEIGHT: i32 = 420;

const REFERENCE_WIDTH: i32 = 960;
const REFERENCE_HEIGHT: i32 = 540;

pub enum GameState {
    Action,
    Automap,
}

pub struct Game {
    pub primary_window: VirtualWindow,
    pub game_state: GameState,
    pub renderer: Renderer,
    pub camera: Camera,
    pub player: Player,
    pub world: World,
    pub triangles: Vec<(Vertex, Vertex, Vertex)>,
    pub texture: Buffer2D,
    pub conchars: Option<Buffer2D>,
    pub x: i32,
    pub y: i32,
}

impl Game {
    pub fn new() -> Self {
        let conchars = bmp::load_bmp("./assets/conchars.bmp").expect("no such bmp file");

        let wad = wad::load("./assets/DOOM1.WAD").expect("no such wad file");
        let map_data = wad.get_map_data("E1M1").expect("no such map");

        let renderer = Renderer::new(PRIMARY_WIDTH as usize, PRIMARY_HEIGHT as usize);
        let mut camera = Camera::new();
        camera.set_perspective(
            f32::to_radians(90.0),
            PRIMARY_WIDTH as f32 / PRIMARY_HEIGHT as f32,
            0.01,
            100.0,
        );

        let mut triangles: Vec<(Vertex, Vertex, Vertex)> = vec![];
        for x in 0..5 {
            for z in 0..5 {
                let offset_x = 0.5 * x as f32 + 1.25;
                let offset_z = 0.5 * z as f32 + 1.25;
                let offset_y = 0.1;
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

        let primary_window_x = ((REFERENCE_WIDTH - PRIMARY_WIDTH) / 2) as i32;
        let primary_window_y = ((REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2) as i32;

        Self {
            primary_window: VirtualWindow::new(
                primary_window_x,
                primary_window_y,
                PRIMARY_WIDTH as usize,
                PRIMARY_HEIGHT as usize,
            ),
            game_state: GameState::Action,
            renderer: renderer,
            camera: camera,
            player: Player::new(),
            world: World::new(map_data),
            triangles: triangles,
            texture: wad.load_texture_into_buffer("WALL03_7"),
            conchars: Some(conchars),
            x: 0,
            y: 0,
        }
    }
}

impl Application for Game {
    fn get_title(&self) -> &'static str {
        "Almanac X"
    }

    fn main_loop(&mut self, input: &Input, dt: f32, buffer: Option<&mut Buffer2DSlice>) -> bool {
        if input.is_pressed(27) {
            return false;
        }

        self.player.handle_input(
            input.is_held(vk!('W')),
            input.is_held(vk!('S')),
            input.is_held(vk!('A')),
            input.is_held(vk!('D')),
            input.is_held(0x25),
            input.is_held(0x27),
            input.is_held(16),
        );

        // if input.key_held(VirtualKeyCode::Up) {
        //     self.y -= 3;
        // }
        // if input.key_held(VirtualKeyCode::Down) {
        //     self.y += 3;
        // }
        // if input.key_held(VirtualKeyCode::Right) {
        //     self.x += 3;
        // }
        // if input.key_held(VirtualKeyCode::Left) {
        //     self.x -= 3;
        // }

        // // if input.key_pressed(VirtualKeyCode::F11) {
        // //     println!("{:?}", self.renderer.get_tris_count());
        // // }

        // if input.key_pressed(VirtualKeyCode::Tab) {
        //     self.game_state = match self.game_state {
        //         GameState::Action => GameState::Automap,
        //         _ => GameState::Action,
        //     }
        // }

        // // if input.key_pressed(VirtualKeyCode::Z) {
        // //     self.renderer.set_texture(self.texture.take());
        // // }

        // // if input.key_pressed(VirtualKeyCode::X) {
        // //     self.texture = self.renderer.take_texture();
        // // }

        self.player.update(dt);

        if let Some(buffer) = buffer {
            self.renderer.begin();

            let mut primary_window_target = self.primary_window.get_buffer_slice();
            primary_window_target.clear();
            let mut ctx = self.renderer.create_context_3d(
                self.camera.get_projection() * self.player.get_view(),
                &mut primary_window_target,
            );

            draw_grid(&mut ctx, Vector3::<f32>::zero(), 0.5);

            match self.game_state {
                GameState::Action => {
                    ctx.push_texture(&self.texture);
                    for (v0, v1, v2) in &self.triangles {
                        ctx.draw_triangle(v0, v1, v2);
                    }
                    ctx.pop_texture();
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

            buffer.blit_virtual_window(&self.primary_window);

            if let Some(conchars) = &self.conchars {
                buffer.blit_buffer(conchars, self.x, self.y);
            }
        }

        return true;
    }
}
