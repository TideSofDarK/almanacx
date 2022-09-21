#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod console;
mod game;
mod math;
mod player;
mod renderer;
mod wad;
mod world;

use std::time::Instant;

use console::Console;
use game::Game;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(WIDTH * 2, HEIGHT * 2);
        WindowBuilder::new()
            .with_title("Almanac10")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut game = Game::new();
    let mut console = Console::new(WIDTH, HEIGHT);

    let mut dt = 0.0;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            frame.fill(0x00);
            game.draw(frame);
            console.draw(frame);

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            dt = last_frame.elapsed().as_secs_f32();
            last_frame = Instant::now();

            if input.key_pressed(VirtualKeyCode::Grave) {
                console.toggle();
            }

            if input.key_pressed(VirtualKeyCode::B) {
                println!("{:?}", dt);
            }

            if console.is_open() {
                console.handle_input(&input);
            } else {
                if game.handle_input(&input) {
                    *control_flow = ControlFlow::Exit;
                }
            }

            console.update(dt);
            game.update(dt);

            window.request_redraw();
        }
    });
}
