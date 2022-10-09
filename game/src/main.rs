mod game;
mod player;
mod world;

use common::application::{init_application, Application};
use common::console::Console;
use common::{HEIGHT, WIDTH};
use game::Game;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

struct AlmanacX {
    game: Game,
    console: Console,
}

impl Application for AlmanacX {
    fn get_window_name(&self) -> &'static str {
        "Almanac X"
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Grave) {
            self.console.toggle();
        }

        if self.console.is_open() {
            self.console.handle_input(&input);
        } else {
            self.game.handle_input(&input);
        }
    }

    fn update(&mut self, dt: f32) {
        self.console.update(dt);
        self.game.update(dt);
    }

    fn draw(&mut self, frame: &mut [u8]) {
        self.game.draw(frame);
        self.console.draw(frame);
    }
}

fn main() {
    init_application(
        WIDTH,
        HEIGHT,
        AlmanacX {
            game: Game::new(),
            console: Console::new(WIDTH, HEIGHT),
        },
    )
    .unwrap_or_default()
}
