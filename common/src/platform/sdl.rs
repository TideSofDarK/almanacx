use core::slice;
use std::time::Instant;

use crate::buffer2d::B2DS;

use super::{
    input::{Input, InputCode},
    Application,
};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

pub unsafe fn init_application<A: Application>(app: A) {
    sdl_main(app).unwrap();
}

pub fn sdl_main<A: Application>(mut app: A) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(app.get_title(), 0, 0)
        .fullscreen_desktop()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let surface_width = 1920 / 3;
    let surface_height = 1080 / 3;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 255));

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB1555,
            surface_width as u32,
            surface_height as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut input = Input::new();

    let mut event_pump = sdl_context.event_pump()?;

    let mut previous = Instant::now();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::TextInput { text, .. } => {
                    if let Some(last_char) = text.chars().next() {
                        input.last_char = Some(last_char);
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    input.set_key(sdlkey_to_input_code(keycode), true);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    input.set_key(sdlkey_to_input_code(keycode), false);
                }
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    sdl2::mouse::MouseButton::Left => input.set_key(InputCode::LMB, true),
                    sdl2::mouse::MouseButton::Middle => input.set_key(InputCode::MMB, true),
                    sdl2::mouse::MouseButton::Right => input.set_key(InputCode::RMB, true),
                    _ => {}
                },
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    sdl2::mouse::MouseButton::Left => input.set_key(InputCode::LMB, false),
                    sdl2::mouse::MouseButton::Middle => input.set_key(InputCode::MMB, false),
                    sdl2::mouse::MouseButton::Right => input.set_key(InputCode::RMB, false),
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => input.update_mouse((x / 3, y / 3), (x, y)),
                _ => {}
            }
        }

        let current = Instant::now();
        let dt = (current - previous).as_secs_f32();
        previous = current;

        // canvas.clear();
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| unsafe {
            let bitmap = slice::from_raw_parts_mut(
                buffer.as_mut_ptr() as *mut u16,
                (surface_height * surface_width) as usize,
            );
            app.main_loop(
                &input,
                dt,
                Some(B2DS {
                    width: surface_width,
                    height: surface_height,
                    bitmap,
                }),
            );
        })?;
        canvas.copy(&texture, None, None)?;
        canvas.present();

        input.reset();
    }

    Ok(())
}

pub fn sdlkey_to_input_code(vkey: Keycode) -> InputCode {
    match vkey {
        Keycode::Backslash => InputCode::Back,
        Keycode::Tab => InputCode::Tab,
        Keycode::Return => InputCode::Return,
        Keycode::LShift => InputCode::Shift,
        Keycode::RShift => InputCode::Shift,
        Keycode::LCtrl => InputCode::LControl,
        Keycode::RCtrl => InputCode::RControl,
        Keycode::LAlt => InputCode::LAlt,
        Keycode::RAlt => InputCode::RAlt,
        Keycode::Pause => InputCode::Pause,
        Keycode::Escape => InputCode::Escape,
        Keycode::Space => InputCode::Space,
        Keycode::PageUp => InputCode::PageUp,
        Keycode::PageDown => InputCode::PageDown,
        Keycode::End => InputCode::End,
        Keycode::Home => InputCode::Home,
        Keycode::Left => InputCode::Left,
        Keycode::Up => InputCode::Up,
        Keycode::Right => InputCode::Right,
        Keycode::Down => InputCode::Down,
        Keycode::Insert => InputCode::Insert,
        Keycode::Delete => InputCode::Delete,
        Keycode::Num0 => InputCode::Key0,
        Keycode::Num1 => InputCode::Key1,
        Keycode::Num2 => InputCode::Key2,
        Keycode::Num3 => InputCode::Key3,
        Keycode::Num4 => InputCode::Key4,
        Keycode::Num5 => InputCode::Key5,
        Keycode::Num6 => InputCode::Key6,
        Keycode::Num7 => InputCode::Key7,
        Keycode::Num8 => InputCode::Key8,
        Keycode::Num9 => InputCode::Key9,
        Keycode::A => InputCode::A,
        Keycode::B => InputCode::B,
        Keycode::C => InputCode::C,
        Keycode::D => InputCode::D,
        Keycode::E => InputCode::E,
        Keycode::F => InputCode::F,
        Keycode::G => InputCode::G,
        Keycode::H => InputCode::H,
        Keycode::I => InputCode::I,
        Keycode::J => InputCode::J,
        Keycode::K => InputCode::K,
        Keycode::L => InputCode::L,
        Keycode::M => InputCode::M,
        Keycode::N => InputCode::N,
        Keycode::O => InputCode::O,
        Keycode::P => InputCode::P,
        Keycode::Q => InputCode::Q,
        Keycode::R => InputCode::R,
        Keycode::S => InputCode::S,
        Keycode::T => InputCode::T,
        Keycode::U => InputCode::U,
        Keycode::V => InputCode::V,
        Keycode::W => InputCode::W,
        Keycode::X => InputCode::X,
        Keycode::Y => InputCode::Y,
        Keycode::Z => InputCode::Z,
        Keycode::Application => InputCode::LWin,
        Keycode::Kp0 => InputCode::Numpad0,
        Keycode::Kp1 => InputCode::Numpad1,
        Keycode::Kp2 => InputCode::Numpad2,
        Keycode::Kp3 => InputCode::Numpad3,
        Keycode::Kp4 => InputCode::Numpad4,
        Keycode::Kp5 => InputCode::Numpad5,
        Keycode::Kp6 => InputCode::Numpad6,
        Keycode::Kp7 => InputCode::Numpad7,
        Keycode::Kp8 => InputCode::Numpad8,
        Keycode::Kp9 => InputCode::Numpad9,
        Keycode::KpMultiply => InputCode::NumpadMultiply,
        Keycode::KpPlus => InputCode::NumpadAdd,
        Keycode::KpMinus => InputCode::NumpadSubtract,
        Keycode::KpDivide => InputCode::NumpadDivide,
        Keycode::F1 => InputCode::F1,
        Keycode::F2 => InputCode::F2,
        Keycode::F3 => InputCode::F3,
        Keycode::F4 => InputCode::F4,
        Keycode::F5 => InputCode::F5,
        Keycode::F6 => InputCode::F6,
        Keycode::F7 => InputCode::F7,
        Keycode::F8 => InputCode::F8,
        Keycode::F9 => InputCode::F9,
        Keycode::F10 => InputCode::F10,
        Keycode::F11 => InputCode::F11,
        Keycode::F12 => InputCode::F12,
        Keycode::NumLockClear => InputCode::Numlock,
        Keycode::ScrollLock => InputCode::Scroll,
        Keycode::Backquote => InputCode::Grave,
        _ => InputCode::Invalid,
    }
}
