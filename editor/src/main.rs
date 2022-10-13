mod png;
mod view;

use common::{
    application::{init_application, Application},
    console::Console,
    draw_target::DrawTarget,
};
use winit_input_helper::WinitInputHelper;

struct Quill {
    console: Console,
}

impl Application for Quill {
    fn get_name(&self) -> &'static str {
        "Quill"
    }

    fn handle_input(&mut self, input: &WinitInputHelper) {
        // if input.key_pressed(VirtualKeyCode::Grave) {
        //     self.console.toggle();
        // }
    }

    fn update(&mut self, dt: f32) {
        self.console.update(dt);
    }

    fn draw(&mut self, target: &mut DrawTarget<'_>) {
        // self.console.draw(frame);
    }

    fn resize_window(&mut self, width: u32, height: u32) {}

    fn get_reference_dimensions(&self) -> Option<(u32, u32)> {
        todo!()
    }
}

fn main() {
    // init_application(
    //     640,
    //     360,
    //     Quill {
    //         console: Console::new(640, 360),
    //     },
    // );
}
