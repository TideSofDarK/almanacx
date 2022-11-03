pub mod definitions;
pub mod player;
pub mod world;

use std::time::Instant;

use cgmath::{Vector3, Vector4, Zero};
use common::{
    buffer2d::{
        text::Font,
        virtual_window::{VirtualWindowStack, WindowBorder},
        B2DO, B2DS,
    },
    console::Console,
    platform::{
        input::{Input, InputCode},
        Application,
    },
    renderer::{camera::Camera, utils::draw_grid, RenderDebugMode, Renderer},
    utils::color_from_tuple,
};

use self::{definitions::VW_TEST_A, player::Player, world::World};

pub enum GameState {
    Action,
    Automap,
}

pub struct Game {
    pub console: Console,
    pub stack: VirtualWindowStack,
    pub game_state: GameState,
    pub renderer: Renderer,
    pub camera: Camera,
    pub player: Player,
    pub world: World,
    pub texture: B2DO,
    pub crusader: B2DO,
    pub border: WindowBorder,
    pub font: Font,
    pub x: i32,
    pub y: i32,
    pub tick: f32,
    pub time_start: Instant,
}

impl Application for Game {
    fn get_title(&self) -> &'static str {
        "Almanac X"
    }

    fn main_loop(&mut self, input: &Input, dt: f32, main_buffer: Option<B2DS>) -> bool {
        if input.is_pressed(InputCode::Escape) {
            return false;
        }

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
            println!("{}", self.renderer);
        }

        if input.is_pressed(InputCode::F11) {
            println!("{:?}", dt);
        }

        if input.is_pressed(InputCode::F10) {
            self.renderer.debug_mode = match self.renderer.debug_mode {
                RenderDebugMode::None => RenderDebugMode::ZBuffer,
                RenderDebugMode::ZBuffer => RenderDebugMode::None,
                RenderDebugMode::Clickables => RenderDebugMode::None,
            }
        }

        self.tick += dt;
        if self.tick > 1.1 {
            self.tick = 0.0;
            // self.console.put_string(format!(
            //     "Current Tick Current tick Current tick Current tick: \"{}\"",
            //     std::time::Instant::now()
            //         .duration_since(self.time_start)
            //         .as_secs_f32()
            // ));
        }

        if !self.console.update(dt, input) {
            self.stack.update(input);
            self.player.update(dt, input);
        }

        let test_a = &mut self.stack.windows[VW_TEST_A];
        // test_a.minimized = !input.is_held(InputCode::LMB);

        if let Some(mut main_buffer) = main_buffer {
            main_buffer.bitmap.fill(color_from_tuple((2, 2, 2)));

            self.renderer.begin(self.camera.proj, self.player.view);

            draw_grid(&mut self.renderer, Vector3::<f32>::zero(), 0.5);

            match self.game_state {
                GameState::Action => {
                    for (v0, v1, v2) in &self.world.triangles {
                        self.renderer.draw_triangle(v0, v1, v2, Some(&self.texture));
                    }

                    for i in 0..10 {
                        self.renderer.draw_sprite(
                            Vector4::new(0.25 + (i as f32 * 0.1), 0.0, 1.5 + (i as f32 * 0.4), 1.0),
                            1.0,
                            &self.crusader,
                        );
                    }
                }
                GameState::Automap => {}
            }

            // buffer.blit_buffer(&self.renderer.color_buffer, 0, 0);

            // if !test_a.minimized {
            // let mut test_a_buffer = test_a.buffer.borrow_mut();

            // self.font
            //     .blit_str(&mut test_a_buffer, "!@#$%^&s*()_+", 12, 12);

            //     self.font
            //         .blit_str(&mut test_a_buffer, "1234567890-=", 12, 12 + 8);

            //     self.font
            //         .blit_str(&mut test_a_buffer, "AaBbCcDdEeFfGgHhIiJjKk", 12, 12 + 16);

            //     self.font
            //         .blit_str(&mut test_a_buffer, "LlMmNnOoPpQqRrSsTtUuVv", 12, 12 + 24);

            //     self.font
            //         .blit_str(&mut test_a_buffer, "Ww  Xx Yy Zz", 12, 12 + 32);
            // }

            // main_buffer.blit_buffer_full_masked(&self.crusader, (0, 0));
            self.stack.blit(&self.border, &mut main_buffer);
            self.console.blit(&mut main_buffer);
        }

        return true;
    }
}
