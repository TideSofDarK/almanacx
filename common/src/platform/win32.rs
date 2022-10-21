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
            BITMAPINFOHEADER, DIB_RGB_COLORS, PAINTSTRUCT, SRCCOPY,
        },
        System::{
            LibraryLoader::GetModuleHandleW,
            Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
        },
        UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE},
    },
};

use crate::buffer2d::Buffer2DSlice;

use super::{
    input::{Input, INPUT_LMB},
    Application,
};

struct Win32UserData {
    bitmap_info: BITMAPINFO,
    pixels: *mut u8,
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
    user_data.bitmap_info.bmiHeader.biBitCount = 32;
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
            let color_buffer = slice::from_raw_parts_mut(
                user_data.pixels,
                (user_data.bitmap_info.bmiHeader.biWidth
                    * user_data.bitmap_info.bmiHeader.biHeight
                    * 4) as usize,
            );
            let mut buffer = Buffer2DSlice::new(
                user_data.bitmap_info.bmiHeader.biWidth as u32,
                user_data.bitmap_info.bmiHeader.biHeight as u32,
                color_buffer,
            );

            let current = Instant::now();
            let elapsed = current - previous;
            previous = current;
            accumulator += elapsed.as_secs_f32();

            while accumulator >= MS_PER_UPDATE {
                accumulator -= MS_PER_UPDATE;

                if !app.main_loop(
                    &user_data.input,
                    MS_PER_UPDATE,
                    if accumulator < MS_PER_UPDATE {
                        Some(&mut buffer)
                    } else {
                        None
                    },
                ) {
                    break 'outer;
                }

                user_data.input.cache_previous();
            }

            InvalidateRect(window_handle, std::ptr::null(), 0);
            UpdateWindow(window_handle);
        }
    }

    let _ = Box::from_raw(user_data);
    DestroyWindow(window_handle);
    UnregisterClassW(window_class_name, instance);
}

unsafe fn get_window_dimensions(window: HWND) -> (i32, i32) {
    let mut rect: RECT = mem::zeroed();
    GetClientRect(window, &mut rect);
    return (rect.right - rect.left, rect.bottom - rect.top);
}

unsafe fn resize_surface(data: &mut Win32UserData, window_width: i32, window_height: i32) {
    data.bitmap_info.bmiHeader.biWidth = window_width / 2;
    data.bitmap_info.bmiHeader.biHeight = window_height / 2;

    if !data.pixels.is_null() {
        VirtualFree(data.pixels as *mut c_void, 0, MEM_RELEASE);
    }

    data.pixels = VirtualAlloc(
        std::ptr::null(),
        (data.bitmap_info.bmiHeader.biWidth * data.bitmap_info.bmiHeader.biHeight * 4) as usize,
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
                data.bitmap_info.bmiHeader.biHeight,
                data.pixels as *mut core::ffi::c_void,
                &data.bitmap_info,
                DIB_RGB_COLORS,
                SRCCOPY,
            );
            EndPaint(window_handle, &mut ps);
        }
        WM_LBUTTONDOWN => {
            data.input.set_key(INPUT_LMB, true);
        }
        WM_LBUTTONUP => {
            data.input.set_key(INPUT_LMB, false);
        }
        WM_RBUTTONDOWN => {
            data.input.set_key(INPUT_LMB, true);
        }
        WM_RBUTTONUP => {
            data.input.set_key(INPUT_LMB, false);
        }
        WM_MBUTTONDOWN => {
            data.input.set_key(INPUT_LMB, true);
        }
        WM_MBUTTONUP => {
            data.input.set_key(INPUT_LMB, false);
        }
        WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
            if !data.has_focus {
                return 0;
            }

            let key_is_down = (lparam & (1 << 31)) == 0;
            let key_was_down = (lparam & (1 << 30)) != 0;

            if key_is_down && !key_was_down {
                data.input.set_key(wparam, true);
            } else if key_was_down && !key_is_down {
                data.input.set_key(wparam, false);
            }
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
