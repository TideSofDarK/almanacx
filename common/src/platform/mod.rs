use crate::buffer2d::Buffer2DSlice;

use self::input::Input;

pub mod input;

#[cfg_attr(linux, path = "unix.rs")]
#[cfg_attr(windows, path = "win32.rs")]
mod platform_implementation;

pub trait Application {
    fn get_title(&self) -> &'static str;
    fn main_loop(&mut self, input: &Input, dt: f32, buffer: Option<&mut Buffer2DSlice>) -> bool;
}

pub fn init_application<A: Application>(app: A) {
    unsafe { platform_implementation::init_application(app) }
}
