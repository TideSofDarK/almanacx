use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::draw_target::DrawTarget;

pub trait Application {
    fn get_name(&self) -> &'static str;
    fn handle_input(&mut self, input: &WinitInputHelper);
    fn update(&mut self, dt: f32);
    fn draw(&mut self, target: &mut DrawTarget);
    fn resize_window(&mut self, width: u32, height: u32);
    fn get_reference_dimensions(&self) -> Option<(u32, u32)>;
}

pub fn init_application<A>(mut app: A)
where
    A: 'static + Application,
{
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title(app.get_name())
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let (physical_width, physical_height) = (window.inner_size().width, window.inner_size().height);
    let (reference_width, reference_height) = match app.get_reference_dimensions() {
        Some(a) => a,
        None => (physical_width, physical_height),
    };
    let mut pixels = Pixels::new(
        reference_width,
        reference_height,
        SurfaceTexture::new(physical_width, physical_height, &window),
    )
    .expect("failed to init pixels");

    let frame = pixels.get_frame();
    frame.fill(128);

    let mut dt = 0.0;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            app.draw(&mut DrawTarget::new(
                pixels.get_frame(),
                reference_width,
                reference_height,
            ));

            if pixels.render().is_err() {
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
                // pixels.resize_buffer(size.width, size.height);
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
