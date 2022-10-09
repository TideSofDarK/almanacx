use std::time::Instant;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub trait Application {
    fn get_window_name(&self) -> &'static str;
    fn handle_input(&mut self, input: &WinitInputHelper);
    fn update(&mut self, dt: f32);
    fn draw(&mut self, frame: &mut [u8]);
}

pub fn init_application(
    width: u32,
    height: u32,
    mut app: impl Application + 'static,
) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(width * 2, height * 2);
        WindowBuilder::new()
            .with_title(app.get_window_name())
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    let mut dt = 0.0;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            frame.fill(0x00);
            app.draw(frame);

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
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            dt = last_frame.elapsed().as_secs_f32();
            last_frame = Instant::now();

            if input.key_pressed(VirtualKeyCode::F12) {
                println!("{:?}", dt);
            }

            app.handle_input(&input);
            app.update(dt);

            window.request_redraw();
        }
    });
}
