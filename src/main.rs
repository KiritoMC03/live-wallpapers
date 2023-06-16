//#![windows_subsystem = "windows"]
use rand::Rng;
use rapier2d::prelude::*;

use rapier2d::na::Vector2;
use winapi::um::winuser::{
    MSG,
    SetProcessDPIAware,
    WM_ERASEBKGND,
};

use live_wallpapers::*;

use live::app::*;
use live::physics::*;
use live::graphics::*;
use live::bacteries_processing::*;

pub mod live;

fn main() {
    unsafe {
        SetProcessDPIAware();
        APP_DATA = AppData::lazy();
    }

    let app = mut_app_data();
    app.build_physics();
    app.spawn_bacteries(8..16);
    app.live_data.bacteries.genome.fill_default();
    app.with_edges(100.0, 100.0);

    for i in app.live_data.bacteries.into_iter() {
        let x = rand::thread_rng().gen_range(-100..100) as f32;
        let y = rand::thread_rng().gen_range(-100..100) as f32;
        app.live_data.physics_data.get_rb_mut(app.live_data.bacteries.rigidbody[i]).set_linvel(Vector2::new(x, y), true);
    }

    let window_handle = create_desktop_window_fast("Live", Some(window_procedure));
    let delay = 1_000_000 / 80;
    loop_frames(delay, app, window_handle);
}

fn loop_frames(delay: u64, app: &mut AppData, window_handle: HWND) {
    let msg = MSG::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let graphics_pipeline = GraphicsPipeline::new(handle_window_messages);

    loop { // ToDo: stop on app close
        let frame_start = std::time::Instant::now();

        physics_step(&mut physics_pipeline, &mut app.live_data.physics_data);
        graphics_pipeline.step(msg, app, window_handle);
        process_bacteries(app);

        let elapsed = frame_start.elapsed().as_micros();
        if (elapsed as u64) < delay {
            std::thread::sleep(std::time::Duration::from_micros(delay - elapsed as u64));
        }
    }
}

fn simulate_frame(hwnd: HWND, app: &mut AppData) {
    if app.frame_processed { return }
    app.frame_processed = true;

    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    paint_frame(hdc, &ps, app);
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