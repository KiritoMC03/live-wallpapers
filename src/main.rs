//#![windows_subsystem = "windows"]

use std::ptr::null_mut;

use winapi::um::winuser::{MSG, InvalidateRect, RedrawWindow};
use live_wallpapers::*;


fn main() {
    let class_name = wide_null("Window Class Name");
    let window_name = wide_null("Window Name");
    let (window_class, h_instance) = create_window_class(&class_name/*, Some(window_procedure)*/);
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

        unsafe { InvalidateRect(window_handle, null_mut(), 0) };
        unsafe { RedrawWindow(window_handle, null_mut(), null_mut(), 0) };
    }
}