mod png;
mod view;

use common::{
    application::{init_application, Application},
    console::Console,
    HEIGHT, WIDTH,
};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

struct Quill {
    console: Console,
}

impl Application for Quill {
    fn get_window_name(&self) -> &'static str {
        "Quill"
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Grave) {
            self.console.toggle();
        }
    }

    fn update(&mut self, dt: f32) {
        self.console.update(dt);
    }

    fn draw(&mut self, frame: &mut [u8]) {
        self.console.draw(frame);
    }
}

fn main() {
    init_application(
        WIDTH,
        HEIGHT,
        Quill {
            console: Console::new(WIDTH, HEIGHT),
        },
    )
    .unwrap_or_default()
}
