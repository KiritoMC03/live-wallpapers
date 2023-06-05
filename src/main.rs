//#![windows_subsystem = "windows"]

use std::{ptr::null_mut, thread, time::Duration};

use winapi::um::winuser::{MSG, InvalidateRect, RedrawWindow};
use live_wallpapers::*;


type CUSTOM_RGB = rgb::RGB<u8>;

static mut FRAME : u128 = 0;

fn main() {
    let class_name = wide_null("Window Class Name");
    let window_name = wide_null("Window Name");
    let (window_class, h_instance) = create_window_class(&class_name, Some(window_procedure));
    let window_handle = create_window_handle(&window_class, &class_name, &window_name, h_instance);
    let _window = create_window(window_handle);

    let progman_h = get_progman_handle();
    if try_spawn_worker_w(progman_h).is_err() {
        panic!("`Progman` failed to spawn WorkerW!");
    };

    let dpi_aspect = get_dpi_aspect(window_handle);
    let worker_w_handle = find_worker_w();
    pull_window_to_desktop(window_handle, worker_w_handle, dpi_aspect);

    let msg = MSG::default();
    loop {
        handle_window_messages(msg);

        thread::sleep(Duration::from_millis(50));
        unsafe { InvalidateRect(window_handle, null_mut(), 0) };
        unsafe { RedrawWindow(window_handle, null_mut(), null_mut(), 0) };
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
        WM_PAINT => {
            simulate_frame(hwnd);
        }
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    0
}

fn simulate_frame(hwnd: HWND) {
    let colors = [
        CUSTOM_RGB::new(226, 239, 84),
        CUSTOM_RGB::new(255, 92, 102),
        CUSTOM_RGB::new(98, 72, 213),
        CUSTOM_RGB::new(226, 239, 84),
    ];


    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    let rect = &ps.rcPaint;

    // Fill the background with a specific color (e.g., blue)
    let color = interpolate_colors(&colors, (unsafe { FRAME } % 200 as u128) as f32 / 200_f32);
    let hbrush = unsafe { CreateSolidBrush(color) };
    unsafe { FillRect(hdc, rect, hbrush) };
    unsafe { EndPaint(hwnd, &ps) };
    unsafe { FRAME += 1 };

}

pub fn interpolate_colors(colors: &[CUSTOM_RGB], weight: f32) -> u32 {
    let num_colors = colors.len();
    let segment = 1.0 / (num_colors - 1) as f32;

    // Find the two adjacent colors for the given weight
    let index1 = (weight / segment).floor() as usize;
    let index2 = index1 + 1;

    let color1 = colors[index1];
    let color2 = colors[index2];

    // Calculate the weight within the segment
    let segment_weight = (weight - index1 as f32 * segment) / segment;

    // Interpolate between the two colors
    let r = ((1.0 - segment_weight) * color1.r as f32 + segment_weight * color2.r as f32) as u8;
    let g = ((1.0 - segment_weight) * color1.g as f32 + segment_weight * color2.g as f32) as u8;
    let b = ((1.0 - segment_weight) * color1.b as f32 + segment_weight * color2.b as f32) as u8;

    RGB(r, g, b)
}