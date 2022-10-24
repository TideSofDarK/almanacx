pub mod definitions;
pub mod player;
pub mod windows;
pub mod world;

use cgmath::{Vector3, Zero};
use common::{
    buffer2d::{
        text::{blit_str, Font},
        virtual_window::{VirtualWindow, WindowBorder},
        B2DO, B2DS,
    },
    image::bmp,
    platform::{
        input::{Input, InputCode},
        Application,
    },
    renderer::{camera::Camera, utils::draw_grid, Renderer},
};

use self::{
    definitions::{PRIMARY_HEIGHT, PRIMARY_WIDTH, VW_PRIMARY, VW_TEST_A},
    player::Player,
    windows::{create_virtual_windows, load_border_texture},
    world::World,
};

pub enum GameState {
    Action,
    Automap,
}

pub struct Game {
    pub virtual_windows: Vec<VirtualWindow>,
    pub game_state: GameState,
    pub renderer: Renderer,
    pub camera: Camera,
    pub player: Player,
    pub world: World,
    pub texture: B2DO,
    pub border: WindowBorder,
    pub font: Font,
    pub x: i32,
    pub y: i32,
}

impl Game {
    pub fn new() -> Self {
        let virtual_windows = create_virtual_windows();
        let renderer = Renderer::new(&virtual_windows[VW_PRIMARY].buffer);

        Self {
            virtual_windows: virtual_windows,
            game_state: GameState::Action,
            renderer: renderer,
            camera: Camera::perspective(
                f32::to_radians(90.0),
                PRIMARY_WIDTH as f32 / PRIMARY_HEIGHT as f32,
                0.01,
                100.0,
            ),
            player: Player::new(),
            world: World::new(),
            texture: bmp::load_bmp("./assets/floor.bmp").expect("no such bmp file"),
            border: WindowBorder::new(load_border_texture()),
            font: Font {
                bitmap: bmp::load_bmp("./assets/conchars.bmp").expect("no such bmp file"),
                char_size: 8,
                offset_x: 0,
                offset_y: 2,
            },
            x: 0,
            y: 0,
        }
    }
}

impl Application for Game {
    fn get_title(&self) -> &'static str {
        "Almanac X"
    }

    fn main_loop(&mut self, input: &Input, dt: f32, main_buffer: Option<B2DS>) -> bool {
        if input.is_pressed(InputCode::Escape) {
            return false;
        }

        self.player.handle_input(
            input.is_held(InputCode::W),
            input.is_held(InputCode::S),
            input.is_held(InputCode::A),
            input.is_held(InputCode::D),
            input.is_held(InputCode::Left),
            input.is_held(InputCode::Right),
            input.is_held(InputCode::Shift),
        );

        if input.is_held(InputCode::W) {
            self.y -= 3;
        }
        if input.is_held(InputCode::S) {
            self.y += 3;
        }
        if input.is_held(InputCode::A) {
            self.x -= 3;
        }
        if input.is_held(InputCode::D) {
            self.x += 3;
        }

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

        if input.is_pressed(InputCode::F12) {
            println!("{:?}", self.renderer.tris_count);
        }

        if input.is_pressed(InputCode::Grave) {
            println!("{:?}", dt);
        }

        self.player.update(dt);

        if let Some(mut main_buffer) = main_buffer {
            main_buffer.pixels.fill(0);

            self.renderer.begin(self.camera.proj * self.player.view);

            draw_grid(&mut self.renderer, Vector3::<f32>::zero(), 0.5);

            match self.game_state {
                GameState::Action => {
                    for (v0, v1, v2) in &self.world.triangles {
                        self.renderer.draw_triangle(v0, v1, v2, Some(&self.texture));
                    }
                }
                GameState::Automap => {}
            }

            // buffer.blit_buffer(&self.renderer.color_buffer, 0, 0);

            self.virtual_windows[VW_PRIMARY].blit_with_border(&mut main_buffer, &self.border);

            if input.is_held(InputCode::LMB) {
                blit_str(
                    &mut self.virtual_windows[VW_TEST_A].buffer.borrow_mut(),
                    "!@#$%^&s*()_+",
                    12,
                    12,
                    &self.font,
                );

                blit_str(
                    &mut self.virtual_windows[VW_TEST_A].buffer.borrow_mut(),
                    "1234567890-=",
                    12,
                    12 + 8,
                    &self.font,
                );

                blit_str(
                    &mut self.virtual_windows[VW_TEST_A].buffer.borrow_mut(),
                    "AaBbCcDdEeFfGgHhIiJjKk",
                    12,
                    12 + 16,
                    &self.font,
                );

                blit_str(
                    &mut self.virtual_windows[VW_TEST_A].buffer.borrow_mut(),
                    "LlMmNnOoPpQqRrSsTtUuVv",
                    12,
                    12 + 24,
                    &self.font,
                );

                blit_str(
                    &mut self.virtual_windows[VW_TEST_A].buffer.borrow_mut(),
                    "WwXxYyZz",
                    12,
                    12 + 32,
                    &self.font,
                );

                self.virtual_windows[VW_TEST_A].blit_with_border(&mut main_buffer, &self.border);
            }
        }

        return true;
    }
}
