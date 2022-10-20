use core::slice;
use std::{
    ffi::c_void,
    mem::{self},
    time::Instant,
};

use windows_sys::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::{
        Graphics::Gdi::{
            BeginPaint, EndPaint, InvalidateRect, StretchDIBits, UpdateWindow, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, PAINTSTRUCT, RGBQUAD, SRCCOPY,
        },
        System::{
            LibraryLoader::GetModuleHandleW,
            Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
        },
        UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
    },
};

use crate::buffer2d::Buffer2DSlice;

use super::{
    input::{Input, INPUT_LMB},
    Application,
};

static mut HAS_FOCUS: bool = false;
static mut BUFFER_PIXELS: *mut u8 = core::ptr::null_mut();
static mut BUFFER_WIDTH: i32 = 0;
static mut BUFFER_HEIGHT: i32 = 0;
static mut INPUT: Input = Input::new();

static mut FRAME_BITMAP_INFO: BITMAPINFO = BITMAPINFO {
    bmiHeader: BITMAPINFOHEADER {
        biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: 0,
        biHeight: 0,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    },
    bmiColors: [RGBQUAD {
        rgbBlue: 0,
        rgbGreen: 0,
        rgbRed: 0,
        rgbReserved: 0,
    }],
};

pub unsafe fn init_application<A: Application>(mut app: A) {
    let instance = GetModuleHandleW(std::ptr::null());
    debug_assert!(instance != 0);

    SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

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

    let mut window_title = app.get_title().encode_utf16().collect::<Vec<u16>>();
    window_title.push('\n' as u16);
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
        std::ptr::null(),
    );
    debug_assert!(window_handle != 0);

    let mut msg: MSG = std::mem::zeroed();

    let mut previous = Instant::now();
    let mut accumulator: f32 = 0.0;

    const MS_PER_UPDATE: f32 = 1.0 / 45.0;

    'outer: while msg.message != WM_QUIT {
        if PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        } else {
            let color_buffer = slice::from_raw_parts_mut(
                BUFFER_PIXELS,
                (BUFFER_WIDTH * BUFFER_HEIGHT * 4) as usize,
            );
            let mut buffer =
                Buffer2DSlice::new(BUFFER_WIDTH as u32, BUFFER_HEIGHT as u32, color_buffer);

            let current = Instant::now();
            let elapsed = current - previous;
            previous = current;
            accumulator += elapsed.as_secs_f32();

            while accumulator >= MS_PER_UPDATE {
                accumulator -= MS_PER_UPDATE;

                if !app.main_loop(
                    &INPUT,
                    MS_PER_UPDATE,
                    if accumulator < MS_PER_UPDATE {
                        Some(&mut buffer)
                    } else {
                        None
                    },
                ) {
                    break 'outer;
                }

                INPUT.cache_previous();
            }

            InvalidateRect(window_handle, std::ptr::null(), 0);
            UpdateWindow(window_handle);
        }
    }

    DestroyWindow(window_handle);
    UnregisterClassW(window_class_name, instance);
}

unsafe fn get_window_dimensions(window: HWND) -> (i32, i32) {
    let mut rect: RECT = mem::zeroed();
    GetClientRect(window, &mut rect);
    return (rect.right - rect.left, rect.bottom - rect.top);
}

unsafe fn resize_buffer(width: i32, height: i32) {
    BUFFER_WIDTH = width / 2;
    BUFFER_HEIGHT = height / 2;

    FRAME_BITMAP_INFO.bmiHeader.biWidth = BUFFER_WIDTH;
    FRAME_BITMAP_INFO.bmiHeader.biHeight = BUFFER_HEIGHT;

    if !BUFFER_PIXELS.is_null() {
        VirtualFree(BUFFER_PIXELS as *mut c_void, 0, MEM_RELEASE);
    }

    BUFFER_PIXELS = VirtualAlloc(
        std::ptr::null(),
        (BUFFER_WIDTH * BUFFER_HEIGHT * 4) as usize,
        MEM_COMMIT,
        PAGE_READWRITE,
    ) as *mut u8;
}

unsafe extern "system" fn main_window_callback(
    window_handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
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
                BUFFER_WIDTH,
                BUFFER_HEIGHT,
                BUFFER_PIXELS as *mut core::ffi::c_void,
                &FRAME_BITMAP_INFO,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            EndPaint(window_handle, &mut ps);
        }
        WM_LBUTTONDOWN => {
            INPUT.set_key(INPUT_LMB, true);
        }
        WM_LBUTTONUP => {
            INPUT.set_key(INPUT_LMB, false);
        }
        WM_RBUTTONDOWN => {
            INPUT.set_key(INPUT_LMB, true);
        }
        WM_RBUTTONUP => {
            INPUT.set_key(INPUT_LMB, false);
        }
        WM_MBUTTONDOWN => {
            INPUT.set_key(INPUT_LMB, true);
        }
        WM_MBUTTONUP => {
            INPUT.set_key(INPUT_LMB, false);
        }
        WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
            if !HAS_FOCUS {
                return 0;
            }

            let key_is_down = (lparam & (1 << 31)) == 0;
            let key_was_down = (lparam & (1 << 30)) != 0;

            if key_is_down && !key_was_down {
                INPUT.set_key(wparam, true);
            } else if key_was_down && !key_is_down {
                INPUT.set_key(wparam, false);
            }
        }
        WM_KILLFOCUS => {
            HAS_FOCUS = false;
            INPUT = mem::zeroed();
        }
        WM_SETFOCUS => {
            HAS_FOCUS = true;
        }
        WM_SIZE => {
            let dimensions = get_window_dimensions(window_handle);
            resize_buffer(dimensions.0, dimensions.1);
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
