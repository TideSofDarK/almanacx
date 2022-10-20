#[macro_export]
macro_rules! vk {
    ($c:literal) => {
        ($c) as usize
    };
}

pub const INPUT_LMB: usize = 253;
pub const INPUT_RMB: usize = 254;
pub const INPUT_MMB: usize = 255;

pub struct Input {
    keys: [bool; 256],
    keys_previous: [bool; 256],
    mouse_x: i32,
    mouse_y: i32,
}

impl Input {
    pub const fn new() -> Self {
        Self {
            keys: [false; 256],
            keys_previous: [false; 256],
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key] && !self.keys_previous[key]
    }

    pub fn is_held(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn is_released(&self, key: usize) -> bool {
        !self.keys[key] && self.keys_previous[key]
    }

    pub fn get_mouse_x(&self) -> i32 {
        self.mouse_x
    }

    pub fn get_mouse_y(&self) -> i32 {
        self.mouse_y
    }

    pub fn cache_previous(&mut self) {
        self.keys_previous = self.keys;
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }

    pub fn set_mouse_x(&mut self, x: i32) {
        self.mouse_x = x
    }

    pub fn set_mouse_y(&mut self, y: i32) {
        self.mouse_y = y
    }
}
