//#![windows_subsystem = "windows"]
use std::ptr::null_mut;

use micromath::vector::F32x2;
use winapi::um::winuser::{
    MSG,
    RedrawWindow,
    GetCursorPos,
    RDW_INVALIDATE, SetProcessDPIAware,
};

use winapi::um::wingdi::TextOutW;
use winapi::shared::windef::{POINT, HDC};

use live_wallpapers::*;
use live_wallpapers::drawing::{
    beauty_math::*,
    primitives::*,
    colors::*,
};

use live::bacteries::*;

pub mod live;

static mut APP_DATA : AppData = AppData{
    width: 0,
    height: 0,
    frame_num: 0,
    frame_processed: false,
    current_galaxy: Galaxy::empty(),
    bg_progress: 0,
};

/// Ignore DPI.
struct AppData {
    width: usize,
    height: usize,
    frame_num: u128,
    frame_processed: bool,
    current_galaxy: Galaxy,
    bg_progress: u128,
}

fn main() {
    unsafe {
        SetProcessDPIAware();
        APP_DATA = AppData {
            width: GetSystemMetrics(SM_CXSCREEN) as usize,
            height: GetSystemMetrics(SM_CYSCREEN) as usize,
            frame_num: 0,
            frame_processed: false,
            current_galaxy: Galaxy::empty(),
            bg_progress: 0,
        };
    }
    let window_handle = create_desktop_window_fast("Live", Some(window_procedure));

    let msg = MSG::default();
    let app_data = ref_app_data();
    loop { // ToDo: stop on app close
        if handle_window_messages(msg) { }
        else if !app_data.frame_processed {
            unsafe { RedrawWindow(window_handle, null_mut(), null_mut(), RDW_INVALIDATE); }
        }

        std::thread::sleep(std::time::Duration::from_micros(16666));
    }
}

fn ref_app_data() -> &'static AppData {
    unsafe { &APP_DATA }
}

fn mut_app_data() -> &'static mut AppData {
    unsafe { &mut APP_DATA }
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
        WM_PAINT => simulate_frame(hwnd, mut_app_data()),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    0
}

fn simulate_frame(hwnd: HWND, app: &mut AppData) {
    if app.frame_processed {
        return;
    }
    app.frame_processed = true;

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

    let color = interpolate_colors(&colors, (app.bg_progress % 200 as u128) as f32 / 200_f32);
    let bactery_color = random_color();
    draw_fullscreen_rect(hdc, &ps, color);
    let bacteries = Bacteries::rand_in_rect(100, 0.0, app.width as f32, 0.0, app.height as f32);
    bacteries.draw(|pos| draw_circle(pos, hdc, bactery_color));

    unsafe {
        EndPaint(hwnd, &ps);
        app.frame_num += 1;
        app.frame_processed = false;
    };
}

fn draw_circle(pos: F32x2, hdc: HDC, color: u32) {
    draw_circle_brush(pos.x as i32, pos.y as i32, 4, hdc, color);
}