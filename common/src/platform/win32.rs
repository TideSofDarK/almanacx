use core::slice;
use std::{ffi::c_void, mem, time::Instant};

use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::{
        Graphics::Gdi::{
            BeginPaint, EndPaint, InvalidateRect, StretchDIBits, UpdateWindow, BITMAPINFO,
            BITMAPINFOHEADER, DIB_RGB_COLORS, PAINTSTRUCT, SRCCOPY,
        },
        System::{
            LibraryLoader::GetModuleHandleW,
            Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
        },
        UI::{
            HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE},
            Input::KeyboardAndMouse::{VK_OEM_3, VK_SHIFT},
        },
    },
};

use windows_sys::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::Input::KeyboardAndMouse::{
        VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_ADD,
        VK_APPS, VK_B, VK_BACK, VK_BROWSER_BACK, VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD,
        VK_BROWSER_HOME, VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CAPITAL,
        VK_CONVERT, VK_D, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_ESCAPE, VK_F,
        VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18, VK_F19,
        VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8,
        VK_F9, VK_G, VK_H, VK_HOME, VK_I, VK_INSERT, VK_J, VK_K, VK_KANA, VK_KANJI, VK_L,
        VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN,
        VK_M, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP,
        VK_MULTIPLY, VK_N, VK_NEXT, VK_NONCONVERT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2,
        VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O,
        VK_OEM_102, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_P, VK_PAUSE,
        VK_PRIOR, VK_Q, VK_R, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_S,
        VK_SCROLL, VK_SLEEP, VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V,
        VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP, VK_W, VK_X, VK_Y, VK_Z,
    },
};

use crate::buffer2d::B2DS;

use super::{
    input::{Input, InputCode},
    Application,
};

struct Win32UserData {
    bitmap_info: BITMAPINFO,
    pixels: *mut u16,
    has_focus: bool,
    input: Input,
}

pub unsafe fn init_application<A: Application>(mut app: A) {
    let instance = GetModuleHandleW(std::ptr::null());
    debug_assert!(instance != 0);

    SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);

    let window_class_name = w!("window");
    let window_class = WNDCLASSW {
        hCursor: LoadCursorW(0, IDC_ARROW),
        hInstance: instance,
        lpszClassName: window_class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(main_window_callback),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: 0,
        hbrBackground: 0,
        lpszMenuName: std::ptr::null(),
    };
    let atom = RegisterClassW(&window_class);
    debug_assert!(atom != 0);

    let screen_width = GetSystemMetrics(SM_CXSCREEN);
    let screen_height = GetSystemMetrics(SM_CYSCREEN);

    let mut user_data: Win32UserData = std::mem::zeroed();
    user_data.bitmap_info.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
    user_data.bitmap_info.bmiHeader.biPlanes = 1;
    user_data.bitmap_info.bmiHeader.biBitCount = 16;
    let user_data_box = Box::new(user_data);
    let user_data_ptr = Box::into_raw(user_data_box);
    let user_data = &mut *(user_data_ptr);

    let mut window_title = app.get_title().encode_utf16().collect::<Vec<u16>>();
    window_title.push(0);
    let window_handle = CreateWindowExW(
        0,
        window_class_name,
        window_title.as_ptr(),
        // WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        WS_POPUP | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        screen_width,
        screen_height,
        0,
        0,
        instance,
        user_data_ptr as *const c_void,
    );
    debug_assert!(window_handle != 0);

    let mut previous = Instant::now();
    let mut accumulator: f32 = 0.0;

    const MS_PER_UPDATE: f32 = 1.0 / 45.0;

    let mut msg: MSG = std::mem::zeroed();
    'outer: while msg.message != WM_QUIT {
        if PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        } else {
            let current = Instant::now();
            let elapsed = current - previous;
            previous = current;
            accumulator += elapsed.as_secs_f32();

            while accumulator >= MS_PER_UPDATE {
                let running = if accumulator - MS_PER_UPDATE >= MS_PER_UPDATE {
                    app.main_loop(&user_data.input, MS_PER_UPDATE, None)
                } else {
                    app.main_loop(
                        &user_data.input,
                        MS_PER_UPDATE,
                        Some(B2DS {
                            width: user_data.bitmap_info.bmiHeader.biWidth,
                            height: user_data.bitmap_info.bmiHeader.biHeight.abs(),
                            bitmap: slice::from_raw_parts_mut(
                                user_data.pixels,
                                (user_data.bitmap_info.bmiHeader.biWidth
                                    * user_data.bitmap_info.bmiHeader.biHeight.abs()
                                    * 1) as usize,
                            ),
                        }),
                    )
                };
                if !running {
                    break 'outer;
                }
                accumulator -= MS_PER_UPDATE;
                user_data.input.last_char = None;
                user_data.input.cache_previous();
            }

            InvalidateRect(window_handle, std::ptr::null(), 0);
            UpdateWindow(window_handle);
        }
    }

    DestroyWindow(window_handle);
    UnregisterClassW(window_class_name, instance);
    let _ = Box::from_raw(user_data);
}

unsafe fn get_window_dimensions(window: HWND) -> (i32, i32) {
    let mut rect: RECT = mem::zeroed();
    GetClientRect(window, &mut rect);
    return (rect.right - rect.left, rect.bottom - rect.top);
}

unsafe fn resize_surface(data: &mut Win32UserData, window_width: i32, window_height: i32) {
    data.bitmap_info.bmiHeader.biWidth = window_width / 3;
    data.bitmap_info.bmiHeader.biHeight = -window_height / 3;

    if !data.pixels.is_null() {
        VirtualFree(data.pixels as *mut c_void, 0, MEM_RELEASE);
    }

    data.pixels = VirtualAlloc(
        std::ptr::null(),
        (data.bitmap_info.bmiHeader.biWidth * data.bitmap_info.bmiHeader.biHeight.abs() * 2)
            as usize,
        MEM_COMMIT,
        PAGE_READWRITE,
    ) as *mut u16;
}

unsafe extern "system" fn main_window_callback(
    window_handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_CREATE {
        let ptr = std::mem::transmute::<*mut c_void, *mut Win32UserData>(
            (*std::mem::transmute::<LPARAM, *mut CREATESTRUCTW>(lparam)).lpCreateParams,
        );
        SetWindowLongPtrW(window_handle, GWLP_USERDATA, ptr as *const _ as _);

        return 0;
    }

    let data = &mut *(GetWindowLongPtrW(window_handle, GWLP_USERDATA) as *mut Win32UserData);

    match message {
        WM_ERASEBKGND => {
            return 1;
        }
        WM_PAINT => {
            let dimensions = get_window_dimensions(window_handle);
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(window_handle, &mut ps);
            StretchDIBits(
                hdc,
                0,
                0,
                dimensions.0,
                dimensions.1,
                0,
                0,
                data.bitmap_info.bmiHeader.biWidth,
                data.bitmap_info.bmiHeader.biHeight.abs(),
                data.pixels as *mut core::ffi::c_void,
                &data.bitmap_info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            EndPaint(window_handle, &mut ps);
        }
        WM_LBUTTONDOWN => {
            data.input.set_key(InputCode::LMB, true);
        }
        WM_LBUTTONUP => {
            data.input.set_key(InputCode::LMB, false);
        }
        WM_RBUTTONDOWN => {
            data.input.set_key(InputCode::RMB, true);
        }
        WM_RBUTTONUP => {
            data.input.set_key(InputCode::RMB, false);
        }
        WM_MBUTTONDOWN => {
            data.input.set_key(InputCode::MMB, true);
        }
        WM_MBUTTONUP => {
            data.input.set_key(InputCode::MMB, false);
        }
        WM_MOUSEMOVE => {
            if !data.has_focus {
                return 0;
            }

            data.input.mouse_x = ((lparam & 0xffff) as i32) / 3;
            data.input.mouse_y = (((lparam >> 16) & 0xffff) as i32) / 3;

            // dbg!(data.input.mouse_x, data.input.mouse_y);
        }
        WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
            if !data.has_focus {
                return 0;
            }

            let key_is_down = (lparam & (1 << 31)) == 0;
            let key_was_down = (lparam & (1 << 30)) != 0;

            // dbg!(wparam);

            if key_is_down && !key_was_down {
                data.input
                    .set_key(vkey_to_input_code(wparam as VIRTUAL_KEY), true);
            } else if key_was_down && !key_is_down {
                data.input
                    .set_key(vkey_to_input_code(wparam as VIRTUAL_KEY), false);
            }
        }
        WM_CHAR => {
            if !data.has_focus {
                return 0;
            }

            // dbg!(wparam);

            data.input.last_char = Some(wparam as u8 as char);
        }
        WM_KILLFOCUS => {
            data.has_focus = false;
            data.input = mem::zeroed();
        }
        WM_SETFOCUS => {
            data.has_focus = true;
        }
        WM_SIZE => {
            let dimensions = get_window_dimensions(window_handle);
            resize_surface(data, dimensions.0, dimensions.1);
        }
        WM_DESTROY | WM_QUIT => {
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcW(window_handle, message, wparam, lparam);
        }
    };

    0
}

pub fn vkey_to_input_code(vkey: VIRTUAL_KEY) -> InputCode {
    match vkey {
        VK_BACK => InputCode::Back,
        VK_TAB => InputCode::Tab,
        VK_RETURN => InputCode::Return,
        VK_SHIFT => InputCode::Shift,
        VK_LSHIFT => InputCode::Shift,
        VK_RSHIFT => InputCode::Shift,
        VK_LCONTROL => InputCode::LControl,
        VK_RCONTROL => InputCode::RControl,
        VK_LMENU => InputCode::LAlt,
        VK_RMENU => InputCode::RAlt,
        VK_PAUSE => InputCode::Pause,
        VK_CAPITAL => InputCode::Capital,
        VK_KANA => InputCode::Kana,
        VK_KANJI => InputCode::Kanji,
        VK_ESCAPE => InputCode::Escape,
        VK_CONVERT => InputCode::Convert,
        VK_NONCONVERT => InputCode::NoConvert,
        VK_SPACE => InputCode::Space,
        VK_PRIOR => InputCode::PageUp,
        VK_NEXT => InputCode::PageDown,
        VK_END => InputCode::End,
        VK_HOME => InputCode::Home,
        VK_LEFT => InputCode::Left,
        VK_UP => InputCode::Up,
        VK_RIGHT => InputCode::Right,
        VK_DOWN => InputCode::Down,
        VK_SNAPSHOT => InputCode::Snapshot,
        VK_INSERT => InputCode::Insert,
        VK_DELETE => InputCode::Delete,
        VK_0 => InputCode::Key0,
        VK_1 => InputCode::Key1,
        VK_2 => InputCode::Key2,
        VK_3 => InputCode::Key3,
        VK_4 => InputCode::Key4,
        VK_5 => InputCode::Key5,
        VK_6 => InputCode::Key6,
        VK_7 => InputCode::Key7,
        VK_8 => InputCode::Key8,
        VK_9 => InputCode::Key9,
        VK_A => InputCode::A,
        VK_B => InputCode::B,
        VK_C => InputCode::C,
        VK_D => InputCode::D,
        VK_E => InputCode::E,
        VK_F => InputCode::F,
        VK_G => InputCode::G,
        VK_H => InputCode::H,
        VK_I => InputCode::I,
        VK_J => InputCode::J,
        VK_K => InputCode::K,
        VK_L => InputCode::L,
        VK_M => InputCode::M,
        VK_N => InputCode::N,
        VK_O => InputCode::O,
        VK_P => InputCode::P,
        VK_Q => InputCode::Q,
        VK_R => InputCode::R,
        VK_S => InputCode::S,
        VK_T => InputCode::T,
        VK_U => InputCode::U,
        VK_V => InputCode::V,
        VK_W => InputCode::W,
        VK_X => InputCode::X,
        VK_Y => InputCode::Y,
        VK_Z => InputCode::Z,
        VK_LWIN => InputCode::LWin,
        VK_RWIN => InputCode::RWin,
        VK_APPS => InputCode::Apps,
        VK_SLEEP => InputCode::Sleep,
        VK_NUMPAD0 => InputCode::Numpad0,
        VK_NUMPAD1 => InputCode::Numpad1,
        VK_NUMPAD2 => InputCode::Numpad2,
        VK_NUMPAD3 => InputCode::Numpad3,
        VK_NUMPAD4 => InputCode::Numpad4,
        VK_NUMPAD5 => InputCode::Numpad5,
        VK_NUMPAD6 => InputCode::Numpad6,
        VK_NUMPAD7 => InputCode::Numpad7,
        VK_NUMPAD8 => InputCode::Numpad8,
        VK_NUMPAD9 => InputCode::Numpad9,
        VK_MULTIPLY => InputCode::NumpadMultiply,
        VK_ADD => InputCode::NumpadAdd,
        VK_SUBTRACT => InputCode::NumpadSubtract,
        VK_DECIMAL => InputCode::NumpadDecimal,
        VK_DIVIDE => InputCode::NumpadDivide,
        VK_F1 => InputCode::F1,
        VK_F2 => InputCode::F2,
        VK_F3 => InputCode::F3,
        VK_F4 => InputCode::F4,
        VK_F5 => InputCode::F5,
        VK_F6 => InputCode::F6,
        VK_F7 => InputCode::F7,
        VK_F8 => InputCode::F8,
        VK_F9 => InputCode::F9,
        VK_F10 => InputCode::F10,
        VK_F11 => InputCode::F11,
        VK_F12 => InputCode::F12,
        VK_F13 => InputCode::F13,
        VK_F14 => InputCode::F14,
        VK_F15 => InputCode::F15,
        VK_F16 => InputCode::F16,
        VK_F17 => InputCode::F17,
        VK_F18 => InputCode::F18,
        VK_F19 => InputCode::F19,
        VK_F20 => InputCode::F20,
        VK_F21 => InputCode::F21,
        VK_F22 => InputCode::F22,
        VK_F23 => InputCode::F23,
        VK_F24 => InputCode::F24,
        VK_NUMLOCK => InputCode::Numlock,
        VK_SCROLL => InputCode::Scroll,
        VK_BROWSER_BACK => InputCode::NavigateBackward,
        VK_BROWSER_FORWARD => InputCode::NavigateForward,
        VK_BROWSER_REFRESH => InputCode::WebRefresh,
        VK_BROWSER_STOP => InputCode::WebStop,
        VK_BROWSER_SEARCH => InputCode::WebSearch,
        VK_BROWSER_FAVORITES => InputCode::WebFavorites,
        VK_BROWSER_HOME => InputCode::WebHome,
        VK_VOLUME_MUTE => InputCode::Mute,
        VK_VOLUME_DOWN => InputCode::VolumeDown,
        VK_VOLUME_UP => InputCode::VolumeUp,
        VK_MEDIA_NEXT_TRACK => InputCode::NextTrack,
        VK_MEDIA_PREV_TRACK => InputCode::PrevTrack,
        VK_MEDIA_STOP => InputCode::MediaStop,
        VK_MEDIA_PLAY_PAUSE => InputCode::PlayPause,
        VK_LAUNCH_MAIL => InputCode::Mail,
        VK_LAUNCH_MEDIA_SELECT => InputCode::MediaSelect,
        VK_OEM_PLUS => InputCode::Equals,
        VK_OEM_COMMA => InputCode::Comma,
        VK_OEM_MINUS => InputCode::Minus,
        VK_OEM_PERIOD => InputCode::Period,
        VK_OEM_102 => InputCode::OEM102,
        VK_OEM_3 => InputCode::Grave,
        _ => InputCode::Invalid,
    }
}
