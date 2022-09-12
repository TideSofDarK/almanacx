#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod math;
mod renderer;

use std::f32::consts::PI;

use log::error;
use math::Vertex;
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cgmath::Vector3;

use crate::renderer::{Renderer};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

struct World {
    angle: f32,
    x: f32,
    y: f32,
    renderer: Renderer
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * 2u32) as f64, (HEIGHT * 2u32) as f64);
        WindowBuilder::new()
            .with_title("Almanac10")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    fn new() -> Self {
        Self {
            angle: 0.0,
            x: 0.0,
            y: 0.0,
            renderer: Renderer::new(WIDTH as i16, HEIGHT as i16)
        }
    }

    fn update(&mut self) {
        const length: f32 = 15.0;
        const step: f32 = 0.05;
        if self.angle >= PI * 2.0 {
            self.angle = 0.0;
        }
        self.x = length * self.angle.cos();
        self.y = length * self.angle.sin();
        self.angle += step;
    }

    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }

        // let v0 = Vertex{pos: Vector3::new(50.0, 100.0, 0.0), color: Vector3::new(0.0,0.5,1.0)};
        // let v1 = Vertex{pos: Vector3::new(150.0, 150.0, 0.0), color: Vector3::new(0.0,1.0,0.5)};
        // let v2 = Vertex{pos: Vector3::new(75.0, 75.0, 0.0), color: Vector3::new(1.0,0.5,0.0)};

        // self.renderer.draw_triangle(v0, v1, v2, frame);

        let v0 = Vertex{pos: Vector3::new(250.0 + self.x, 150.0 + self.y, 0.0), color: Vector3::new(1.0,0.0,0.0)};
        let v1 = Vertex{pos: Vector3::new(350.0 + self.x, 220.0 + self.y, 0.0), color: Vector3::new(0.0,0.0,1.0)};
        let v2 = Vertex{pos: Vector3::new(275.0 + self.y, 115.0 + self.x, 0.0), color: Vector3::new(0.0,1.0,0.0)};

        self.renderer.draw_triangle(v0, v1, v2, frame);
    }
}