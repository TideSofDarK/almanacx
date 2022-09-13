#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod math;
mod renderer;
mod player;

use std::f32::consts::PI;

use log::error;
use math::Vertex;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use cgmath::{Vector3, Vector4, Point3};

use crate::renderer::{Renderer};
use crate::player::{Player};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

struct Game {
    angle: f32,
    x_circling: f32,
    y_circling: f32,
    renderer: Renderer,
    player: Player
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

    let mut game = Game::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            game.draw(pixels.get_frame());
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

            game.player.handle_input(
                input.key_held(VirtualKeyCode::W),
                input.key_held(VirtualKeyCode::S),
                input.key_held(VirtualKeyCode::A),
                input.key_held(VirtualKeyCode::D),
                input.key_held(VirtualKeyCode::Left),
                input.key_held(VirtualKeyCode::Right));

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            game.update();
            window.request_redraw();
        }
    });
}

impl Game {
    fn new() -> Self {
        Self {
            angle: 0.0,
            x_circling: 0.0,
            y_circling: 0.0,
            renderer: Renderer::new(WIDTH as i16, HEIGHT as i16),
            player: Player::new()
        }
    }

    fn update(&mut self) {
        const length: f32 = 15.0;
        const step: f32 = 0.05;
        if self.angle >= PI * 2.0 {
            self.angle = 0.0;
        }
        self.x_circling = length * self.angle.cos();
        self.y_circling = length * self.angle.sin();
        self.angle += step;
    }

    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        for (_i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }

        self.renderer.begin(self.player.get_view_matrix());

        for i in 0..10 {
            let offset = (i as f32) * 0.5;
            let offset_y = (i as f32) * 0.25;
            let v0 = Vertex{pos: Vector4::new(-0.25, 0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(1.0,0.0,0.0)};
            let v1 = Vertex{pos: Vector4::new(-0.25, -0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(0.0,1.0,0.0)};
            let v2 = Vertex{pos: Vector4::new(0.25, -0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(0.0,0.0,1.0)};
    
            self.renderer.draw_triangle(v0, v1, v2, frame);
    
            let v0 = Vertex{pos: Vector4::new(-0.25, 0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(1.0,0.0,0.0)};
            let v1 = Vertex{pos: Vector4::new(0.25, 0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(0.0,1.0,0.0)};
            let v2 = Vertex{pos: Vector4::new(0.25, -0.25 + offset_y, 0.0 + offset, 1.0), color: Vector3::new(0.0,0.0,1.0)};
    
            self.renderer.draw_triangle(v0, v1, v2, frame);
        }
    }
}