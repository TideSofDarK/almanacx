pub enum InputCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Snapshot,
    Scroll,
    Pause,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Back,
    Return,
    Space,

    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    Shift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,

    LMB,
    RMB,
    MMB,

    Invalid,
}

pub struct Input {
    keys: [bool; 256],
    keys_previous: [bool; 256],
    pub mouse_x: i32,
    pub mouse_y: i32,
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

    pub fn is_pressed(&self, key: InputCode) -> bool {
        let key = key as usize;
        self.keys[key] && !self.keys_previous[key]
    }

    pub fn is_held(&self, key: InputCode) -> bool {
        let key = key as usize;
        self.keys[key]
    }

    pub fn is_released(&self, key: InputCode) -> bool {
        let key = key as usize;
        !self.keys[key] && self.keys_previous[key]
    }

    pub(super) fn cache_previous(&mut self) {
        self.keys_previous = self.keys;
    }

    pub(super) fn set_key(&mut self, key: InputCode, pressed: bool) {
        let key = key as usize;
        self.keys[key] = pressed;
    }
}
