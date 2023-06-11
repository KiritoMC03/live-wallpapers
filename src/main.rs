//#![windows_subsystem = "windows"]
use std::ptr::null_mut;

use winapi::um::winuser::{
    MSG,
    RedrawWindow,
    GetCursorPos,
    RDW_INVALIDATE,
};

use winapi::um::wingdi::TextOutW;
use winapi::shared::windef::POINT;

use live_wallpapers::*;
use live_wallpapers::drawing::{
    beauty_math::*,
    primitives::*,
    colors::*,
};

static mut FRAME : u128 = 0;
static mut BG_STEPS : u128 = 0;
static mut FRAME_PROCESSED : bool = false;
static mut CURRENT_GALAXY : Galaxy = Galaxy::empty();
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let window_handle = create_desktop_window_fast("Live", Some(window_procedure));

    let msg = MSG::default();
    loop { // ToDo: stop on app close
        if handle_window_messages(msg){ }
        else {
            unsafe {
                if !FRAME_PROCESSED {
                    RedrawWindow(window_handle, null_mut(), null_mut(), RDW_INVALIDATE);
                }
            }
        }
    }
}

pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM,) -> LRESULT {
    match msg {
        WM_NCCREATE => {
            println!("NC Create");
            let createstruct: *mut CREATESTRUCTW = l_param as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr = (*createstruct).lpCreateParams;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
            return 1;
        }
        WM_CREATE => println!("WM Create"),
        WM_CLOSE => drop(DestroyWindow(hwnd)),
        WM_DESTROY => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
            drop(Box::from_raw(ptr));
            println!("Cleaned up the box.");
            PostQuitMessage(0);
        }
        WM_PAINT => simulate_frame(hwnd),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    0
}

fn simulate_frame(hwnd: HWND) {
    unsafe {
        if FRAME_PROCESSED {
            return;
        }
        FRAME_PROCESSED = true;
    }
    let colors = [
        RGB::new(226, 239, 84),
        RGB::new(255, 92, 102),
        RGB::new(98, 72, 213),
        RGB::new(226, 239, 84),
        ];

    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

    let mut p : POINT = POINT::default();
    let ptr = &mut p;
    unsafe { GetCursorPos(ptr) };

    let mouse_x = p.x as f64;
    let mouse_y = p.y as f64;
    unsafe {
        if CURRENT_GALAXY.x != mouse_x || CURRENT_GALAXY.y != mouse_y {
            let color = interpolate_colors(&colors, (BG_STEPS % 200 as u128) as f32 / 200_f32);
            draw_fullscreen_rect(hdc, &ps, color);
            BG_STEPS += 1;
            let c = random_color();
            CURRENT_GALAXY = Galaxy::new(mouse_x, mouse_y, WIDTH, HEIGHT, c);
        }
    }

    let hello = wide_null("Hello, Windows!");
    unsafe { TextOutW(hdc, 0, 0, hello.as_ptr(), 15) };
    draw_galaxy_step_inc(hdc, unsafe { &mut CURRENT_GALAXY });

    unsafe {
        EndPaint(hwnd, &ps);
        FRAME += 1;
        FRAME_PROCESSED = false;
    };
}

