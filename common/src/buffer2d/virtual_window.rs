use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    platform::input::{Input, InputCode},
    utils::is_inside,
};

use super::{B2D, B2DO, B2DS, B2DT};

pub struct WindowBorder {
    pub padding: i32,
    pub size: i32,
    pub offset: i32,
    pub texture: B2DO,
}

impl WindowBorder {
    pub fn new(texture: B2DO) -> Self {
        Self {
            padding: 1,
            size: (texture.width - 1) / 2,
            offset: 6,
            texture: texture,
        }
    }
}

pub struct VirtualWindow {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dragable: bool,
    pub minimized: bool,
    pub buffer: Rc<RefCell<B2DO>>,
}

impl VirtualWindow {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            dragable: true,
            minimized: false,
            buffer: Rc::new(RefCell::new(B2DO::new(width, height))),
        }
    }

    pub fn with_xyz(mut self, x: i32, y: i32, z: i32) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn with_dragable(mut self, dragable: bool) -> Self {
        self.dragable = dragable;
        self
    }

    pub fn blit_with_border<T: B2DT>(&mut self, dest: &mut B2D<T>, border: &WindowBorder) {
        let buffer = self.buffer.borrow();
        dest.blit_full(
            &buffer.pixels,
            (buffer.width, buffer.height),
            (self.x, self.y),
        );

        // Top left
        dest.blit_region_alpha(
            &border.texture.pixels,
            (0, 0),
            (border.size, border.size),
            border.texture.width,
            (self.x - border.offset, self.y - border.offset),
        );

        // Middle
        for x in (self.x - border.offset + border.size)
            ..(self.x + buffer.width - border.size + border.offset)
        {
            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, 0),
                (border.padding, border.size),
                border.texture.width,
                (x, self.y - border.offset),
            );

            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, border.size + border.padding),
                (border.padding, border.size),
                border.texture.width,
                (x, self.y + buffer.height + border.offset - border.size),
            );
        }

        // Top right
        dest.blit_region_alpha(
            &border.texture.pixels,
            (border.size + border.padding, 0),
            (border.size, border.size),
            border.texture.width,
            (
                self.x + buffer.width - border.size + border.offset,
                self.y - border.offset,
            ),
        );

        // Bottom left
        dest.blit_region_alpha(
            &border.texture.pixels,
            (0, border.size + border.padding),
            (border.size, border.size),
            border.texture.width,
            (
                self.x - border.offset,
                self.y + buffer.height - border.size + border.offset,
            ),
        );

        // Bottom right
        dest.blit_region_alpha(
            &border.texture.pixels,
            (border.size + border.padding, border.size + border.padding),
            (border.size, border.size),
            border.texture.width,
            (
                self.x + buffer.width - border.size + border.offset,
                self.y + buffer.height - border.size + border.offset,
            ),
        );

        // Left and right
        for y in (self.y - border.offset + border.size)
            ..(self.y + buffer.height - border.size + border.offset)
        {
            dest.blit_region_alpha(
                &border.texture.pixels,
                (0, border.size + border.padding),
                (border.size, border.padding),
                border.texture.width,
                (self.x - border.offset, y),
            );

            dest.blit_region_alpha(
                &border.texture.pixels,
                (border.size + border.padding, border.size + border.padding),
                (border.size, border.padding),
                border.texture.width,
                (self.x + buffer.width + border.offset - border.size, y),
            );
        }
    }
}

pub struct VirtualWindowStack {
    pub windows: Vec<VirtualWindow>,
    sorted_indices: Vec<(usize, i32)>,
    active_window: usize,
    is_dragging: bool,
    drag_offset: (i32, i32),
}

impl VirtualWindowStack {
    pub fn new(virtual_windows: Vec<VirtualWindow>) -> Self {
        let len = virtual_windows.len();
        Self {
            windows: virtual_windows,
            sorted_indices: vec![(0, 0); len],
            active_window: 0,
            is_dragging: true,
            drag_offset: (0, 0),
        }
    }

    pub fn update(&mut self, input: &Input) {
        if self.is_dragging {
            if input.is_released(InputCode::LMB) || !input.is_held(InputCode::LMB) {
                self.is_dragging = false;
            } else {
                let window = &mut self.windows[self.active_window];

                window.x = self.drag_offset.0 + input.mouse_x;
                window.y = self.drag_offset.1 + input.mouse_y;
            }
        } else {
            if input.is_pressed(InputCode::LMB) {
                self.click_test((input.mouse_x, input.mouse_y));
            }
            self.sort();
        }
    }

    pub fn blit(&mut self, border: &WindowBorder, buffer: &mut B2DS) {
        self.sorted_indices.iter().for_each(|(i, _)| {
            let window = &mut self.windows[*i];
            if !window.minimized {
                window.blit_with_border(buffer, border);
            }
        });
    }

    pub fn get_top_window(&mut self) -> usize {
        self.sorted_indices[0].0
    }

    fn click_test(&mut self, pos: (i32, i32)) -> Option<usize> {
        let mut max_z = 0;
        for (i, _) in self.sorted_indices.iter().rev() {
            let index = *i;
            let window = &mut self.windows[index];
            max_z = window.z.max(max_z);
            let buffer = window.buffer.borrow();
            if is_inside(pos, (window.x, window.y, buffer.width, buffer.height)) {
                window.z = max_z + 1;
                self.is_dragging = true;
                self.drag_offset = (window.x - pos.0, window.y - pos.1);
                self.active_window = index;
                return Some(index);
            }
        }
        return None;
    }

    fn sort(&mut self) {
        self.windows
            .iter()
            .enumerate()
            .for_each(|(i, w)| self.sorted_indices[i] = (i, w.z));
        self.sorted_indices.sort_by(|a, b| a.1.cmp(&b.1));
    }
}
