//#![windows_subsystem = "windows"]
use std::sync::Mutex;

use live::utils::{
    rand_range_vec2,
    rand_ranged_i32,
};

use live::save_load::{load_settings, try_save};
use rand::Rng;

use rapier2d::{
    prelude::*,
    na::Vector2,
};

use winapi::shared::windef::HWND;
use winapi::shared::basetsd::LONG_PTR;

use winapi::shared::minwindef::{
    LPARAM,
    LRESULT,
    UINT,
    WPARAM,
};

use winapi::um::winuser::{
    CREATESTRUCTW,
    PAINTSTRUCT,
    MSG,

    SetProcessDPIAware,
    DefWindowProcW,
    PostQuitMessage,
    DestroyWindow,
    GetWindowLongPtrW,
    SetWindowLongPtrW,
    BeginPaint,
    EndPaint,

    GWLP_USERDATA,
    WM_CLOSE,
    WM_CREATE,
    WM_DESTROY,
    WM_NCCREATE,
    WM_PAINT,
    WM_ERASEBKGND,
};

use wallpaper_app::*;
use wallpaper_app::drawing::colors::*;

use live::app::*;
use live::physics::*;
use live::graphics::*;
use live::bacteries_processing::*;

pub mod live;

fn main() {
    unsafe { SetProcessDPIAware(); }
    build_app();
    let window_handle = create_desktop_window_fast("Live", Some(window_procedure));
    let delay = 1_000_000 / 80;

    loop_logic(delay);
    loop_graphics(delay, &mut mut_app_data(), window_handle);
}

fn build_app() {
    unsafe { APP_DATA = AppData::lazy() };
    let app_mutex = mut_app_data();
    let mut app = app_mutex.lock().unwrap();
    app.live_data.settings = load_settings();
    app.frames_in_day = app.live_data.settings.day_length_sec / app.delta_time;
    app.build_physics();
    let radius = app.live_data.settings.radius_range.clone();
    app.spawn_bacteries(radius);
    app.with_edges(100.0, 100.0);

    for i in app.live_data.bacteries.into_iter() {
        let x = rand::thread_rng().gen_range(-100..100) as f32;
        let y = rand::thread_rng().gen_range(-100..100) as f32;
        let rb = app.live_data.bacteries.rigidbody[i];
        app.live_data.physics_data.get_rb_mut(rb).set_linvel(Vector2::new(x, y), true);
    }
    drop(app);
}

fn loop_graphics(delay: u64, app: &mut Mutex<AppData>, window_handle: HWND) {
    let msg = MSG::default();
    let graphics_pipeline = GraphicsPipeline::new(handle_window_messages);

    loop { // ToDo: stop on app close
        let frame_start = std::time::Instant::now();

        if graphics_pipeline.step(msg, app, window_handle) {
            let elapsed = frame_start.elapsed().as_micros();
            if (elapsed as u64) < delay {
                std::thread::sleep(std::time::Duration::from_micros(delay - elapsed as u64));
            }
        }
    }
}

fn loop_logic(delay: u64) {
    std::thread::spawn(move || {
        let mut physics_pipeline = PhysicsPipeline::new();
        loop {
            let mut app = mut_app_data().lock().unwrap();
            let frame_start = std::time::Instant::now();

            app.day_progress = (app.frame_num as f32 % app.frames_in_day / app.frames_in_day).clamp(0.0, 1.0);
            app.live_data.light_force = interpolate_floats(&app.live_data.settings.light_force, app.day_progress);

            if app.frame_num % 100 == 0 {
                let pos = rand_range_vec2(0.0..app.width as f32, 0.0..app.height as f32);
                let radius = app.live_data.settings.radius_range.clone();
                app.live_data.spawn_bac(pos, rand_ranged_i32(radius));
            }

            physics_step(&mut physics_pipeline, &mut app.live_data.physics_data);
            process_bacteries(&mut app);
//            try_save(&app);
            drop(app);

            let elapsed = frame_start.elapsed().as_micros();
            if (elapsed as u64) < delay {
                std::thread::sleep(std::time::Duration::from_micros(delay - elapsed as u64));
            }
        }
    });
}

fn simulate_frame(hwnd: HWND, app: &mut Mutex<AppData>) {
    let mut app = app.lock().unwrap();
    if app.frame_processed { return }
    app.frame_processed = true;

    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    paint_frame(hdc, &ps, &mut app);
    unsafe { EndPaint(hwnd, &ps) };

    app.frame_num += 1;
    app.frame_processed = false;
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