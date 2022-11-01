use crate::buffer2d::B2DS;

use self::input::Input;

pub mod input;

#[cfg_attr(target_os = "linux", path = "sdl.rs")]
#[cfg_attr(target_os = "windows", path = "win32.rs")]
mod sdl;

pub trait Application {
    fn get_title(&self) -> &'static str;
    fn main_loop(&mut self, input: &Input, dt: f32, buffer: Option<B2DS>) -> bool;
}

pub fn init_application<A: Application>(app: A) {
    unsafe { sdl::init_application(app) }
}
