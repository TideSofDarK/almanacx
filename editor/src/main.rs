mod png;
mod view;

use std::time::Instant;

use common::{application::Application, buffer2d::Buffer2DSlice, console::Console};
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

    fn draw(&mut self, target: &mut Buffer2DSlice<'_>) {
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

    let width = 1920usize;
    let height = 1080usize;
    let mut color_buffer = vec![1; width * height * 1];
    let mut color_buffer_clear = vec![0; width * height * 1];

    // let mut dt = DrawTarget::new(color_buffer.as_mut_slice(), width as u32, height as u32);

    // let conchars = bmp::load_bmp("./assets/conchars.bmp").expect("no such bmp file");

    let mut last_frame = Instant::now();
    color_buffer.fill(0);
    println!("Elapsed1: {:?}", last_frame.elapsed().as_secs_f32());

    let mut last_frame = Instant::now();
    color_buffer.copy_from_slice(&color_buffer_clear);
    println!("Elapsed2: {:?}", last_frame.elapsed().as_secs_f32());

    let mut last_frame = Instant::now();
    for i in 0..width * height * 1 {
        color_buffer[i] = 0;
    }
    println!("Elapsed3: {:?}", last_frame.elapsed().as_secs_f32());
}
