//#![windows_subsystem = "windows"]
use std::ptr::null_mut;
use rand::Rng;

use micromath::vector::F32x2;
use winapi::um::winuser::{
    MSG,
    RedrawWindow,
    GetCursorPos,
    RDW_INVALIDATE, SetProcessDPIAware, WM_ERASEBKGND,
};

use winapi::shared::windef::{POINT, HDC};

use live_wallpapers::*;
use live_wallpapers::drawing::{
    beauty_math::*,
    primitives::*,
    colors::*,
};

use live::*;
use live::bacteries::*;

pub mod live;

static mut APP_DATA : AppData = AppData{
    width: 0,
    height: 0,
    frame_num: 0,
    frame_processed: false,
    delta_time: 0.0,
    current_galaxy: Galaxy::empty(),
    bg_progress: 0,
    live_data: LiveData::empty(),
};

/// Ignore DPI.
struct AppData {
    width: usize,
    height: usize,
    frame_num: u128,
    frame_processed: bool,
    delta_time: f32,
    current_galaxy: Galaxy,
    bg_progress: u128,
    live_data: LiveData,
}

fn main() {
    unsafe {
        SetProcessDPIAware();
        APP_DATA = AppData {
            width: GetSystemMetrics(SM_CXSCREEN) as usize,
            height: GetSystemMetrics(SM_CYSCREEN) as usize,
            frame_num: 0,
            frame_processed: false,
            delta_time: 0.016666,
            current_galaxy: Galaxy::empty(),
            bg_progress: 0,
            live_data: LiveData::default(),
        };

        let radius_range = 4..16;
        let cells_size = (radius_range.end * 3) as f32;
        APP_DATA.live_data.bacteries = Bacteries::rand_in_rect(10, cells_size, 0.0, APP_DATA.width as f32, 0.0, APP_DATA.height as f32);
        APP_DATA.live_data.bacteries.set_random_radius(radius_range.start, radius_range.end);
                for i in APP_DATA.live_data.bacteries.into_iter() {
            APP_DATA.live_data.bacteries.velocity[i] = F32x2 {
                x: rand::thread_rng().gen_range(-300..300) as f32,
                y: rand::thread_rng().gen_range(-300..300) as f32,
            }
        }
    }
    let window_handle = create_desktop_window_fast("Live", Some(window_procedure));

    let msg = MSG::default();
    let app_data = ref_app_data();
    let delay = 1_000_000 / 80;
    loop { // ToDo: stop on app close
        let frame_start = std::time::Instant::now();

        if handle_window_messages(msg) { }
        else if !app_data.frame_processed {
            unsafe { RedrawWindow(window_handle, null_mut(), null_mut(), RDW_INVALIDATE); }
        }

        let elapsed = frame_start.elapsed().as_micros();

        if (elapsed as u64) < delay {
            std::thread::sleep(std::time::Duration::from_micros(delay - elapsed as u64));
        }
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
        WM_ERASEBKGND => return 1,
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

    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    unsafe { GetCursorPos(&mut POINT::default()) };

    paint_frame(hdc, &ps, app);

    unsafe {
        EndPaint(hwnd, &ps);
        app.frame_num += 1;
        app.frame_processed = false;
    };
}

fn paint_frame(hdc: HDC, ps: &PAINTSTRUCT, app: &mut AppData) {
    let colors = [
        RGB::new(226, 239, 84),
        RGB::new(255, 92, 102),
        RGB::new(98, 72, 213),
        RGB::new(226, 239, 84),
    ];

    let color = interpolate_colors(&colors, (app.bg_progress % 200 as u128) as f32 / 200_f32);
    let bactery_color = winapi::um::wingdi::RGB(143, 0, 0);

    let frame = onep_draw_frame(hdc, app.width as i32, app.height as i32);
    draw_fullscreen_rect(frame.hdc, &ps, color);
    app.live_data.bacteries.smart_move(app.delta_time, 0.0, app.width as f32, 0.0, app.height as f32);
    let cs = app.live_data.bacteries.detect_collisions();
    for c in cs {
        println!("{} with {}", c.a, c.b);
    }


    app.live_data.bacteries.draw(|pos, rad| draw_circle(pos, rad, frame.hdc, bactery_color));
    close_draw_frame(hdc, app.width as i32, app.height as i32, frame);
}

fn draw_circle(pos: F32x2, rad: i32, hdc: HDC, color: u32) {
    draw_circle_brush(pos.x as i32, pos.y as i32, rad, hdc, color);
}