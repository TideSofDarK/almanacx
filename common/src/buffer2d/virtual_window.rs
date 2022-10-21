use crate::buffer2d::{B2D, B2DO, B2DS};

use super::B2DT;

pub struct VirtualWindow {
    pub x: i32,
    pub y: i32,
    pub buffer: B2DO,
}

impl VirtualWindow {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: x,
            y: y,
            buffer: B2DO {
                width: width,
                height: height,
                colors: vec![0; (width * height * 4) as usize],
            },
        }
    }

    pub fn get_buffer_slice(&mut self) -> B2DS {
        B2DS {
            width: self.buffer.width,
            height: self.buffer.height,
            colors: &mut self.buffer.colors,
        }
    }
}

impl<T: B2DT> B2D<T> {
    pub fn blit_virtual_window(&mut self, virtual_window: &VirtualWindow) {
        self.blit(
            &virtual_window.buffer.colors,
            virtual_window.buffer.width as i32,
            virtual_window.buffer.height as i32,
            virtual_window.x,
            virtual_window.y,
        )
    }
}
