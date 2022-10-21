use crate::buffer2d::{B2D, B2DO, B2DS, B2DT};

impl<T: B2DT> B2D<T> {
    pub fn blit_char(&mut self, c: char, x: i32, y: i32, conchars: &B2DO) {}
}
