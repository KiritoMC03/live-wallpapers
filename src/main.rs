//#![windows_subsystem = "windows"]

use std::{ptr::null_mut, thread, time::Duration, f64::consts::PI};

use winapi::{um::{winuser::{MSG, InvalidateRect, RedrawWindow, GetDC, GetWindowRect, ReleaseDC, LoadImageW, LR_LOADFROMFILE, IMAGE_BITMAP, LoadImageA, GetClientRect, ValidateRect, GetCursorPos}, wingdi::{SetPixel, DeleteObject, CreatePatternBrush, BITMAPINFO, RGBQUAD, BITMAPINFOHEADER, CreateCompatibleDC, CreateDIBSection, DIB_RGB_COLORS, SelectObject, SRCCOPY, BitBlt, BITMAP, GetObjectW, DeleteDC, CreateCompatibleBitmap, CreatePen, MoveToEx, LineTo, PS_SOLID}, winnt::LONG}, shared::windef::{RECT, HBITMAP, HDC, POINT}, ctypes::c_void};
use live_wallpapers::*;
use live_wallpapers::drawing::*;


type CUSTOM_RGB = rgb::RGB<u8>;

static mut FRAME : u128 = 0;
static mut BG_STEPS : u128 = 0;
static mut FRAME_PROCESSED : bool = false;
static mut ORIG_X: f64 = 0.0;
static mut ORIG_Y: f64 = 0.0;

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

        thread::sleep(Duration::from_micros(16666));
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
            if !FRAME_PROCESSED {
                FRAME_PROCESSED = true;
                simulate_frame(hwnd);
                FRAME_PROCESSED = false;
            }
        }
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    0
}

static mut CURRENT_GALAXY : Galaxy = Galaxy::empty();

fn simulate_frame(hwnd: HWND) {
    let colors = [
        CUSTOM_RGB::new(226, 239, 84),
        CUSTOM_RGB::new(255, 92, 102),
        CUSTOM_RGB::new(98, 72, 213),
        CUSTOM_RGB::new(226, 239, 84),
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
            CURRENT_GALAXY = Galaxy::new(mouse_x, mouse_y, 1920, 1080, c);
        }
    }

    draw_galaxy_step_inc(hdc, unsafe { &mut CURRENT_GALAXY });

    unsafe { EndPaint(hwnd, &ps) };
    unsafe { FRAME += 1 };
}

pub fn random_color() -> u32 {
    RGB(
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    )
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

pub fn draw_spiral(hdc: HDC) {
    let mut angle = 0.0f32;
    let radius_mul = 10.0f32;
    let start_x : f32 = 1920.0 / 2.0;
    let start_y : f32 = 1080.0 / 2.0;

    for i in 0..1000 {
        // Compute radius based on angle
        let radius = angle.powf(0.8);

        // Convert polar coordinates to Cartesian coordinates
        let x = start_x + radius * angle.cos() * radius_mul;
        let y = start_y + radius * angle.sin() * radius_mul;

        let pixel_color = 0xFFFFFF; // white color
        draw_circle_brush(x as i32, y as i32, 3, hdc, pixel_color);

        // Increment the angle for the next iteration
        let c = ((i / 500) as f32).powf(0.4) + 1f32;
        let p = 0.05 / c;
        angle += p;
    }
}

#[derive(Debug, Default)]
struct Galaxy {
    x: f64,
    y: f64,
    color : u32,
    diameter: f64,
    max_diameter: f64,
    curvature: i32,
    theta: f64,
    theta_step: f64,
    is_max_radius: bool,
    hptr_x: f64, // hypotrochoid x anchor (see: http://en.wikipedia.org/wiki/Hypotrochoid)
    hptr_y: f64, // hypotrochoid y anchor
}

impl Galaxy {
    pub fn new(mouse_x: f64, mouse_y: f64, screen_w: i32, screen_h: i32, color: u32) -> Galaxy {
        Galaxy {
            x: mouse_x,
            y: mouse_y,
            color,
            diameter: 9.0,
            max_diameter: 450.0,
            curvature: 10,
            theta: 0.0,
            theta_step: 360.0 * PI / 180.0,
            is_max_radius: false,
            hptr_x: (mouse_x / (screen_w * 999 >> 0) as f64) / 999.0,
            hptr_y: (mouse_y / (screen_h * 999 >> 0) as f64) / 999.0,
        }
    }

    pub const fn empty() -> Galaxy {
        Galaxy {
            x: 0.0,
            y: 0.0,
            color: 0,
            diameter: 0.0,
            max_diameter: 0.0,
            curvature: 0,
            theta: 0.0,
            theta_step: 0.0,
            is_max_radius: false,
            hptr_x: 0.0,
            hptr_y: 0.0,
        }
    }
}

fn draw_galaxy_step_inc(hdc: HDC, galaxy: &mut Galaxy) {
    let mut prev_x = 0.0;
    let mut prev_y = 0.0;
    for curv_step in (0..galaxy.curvature).rev() {
        if galaxy.diameter > galaxy.max_diameter || galaxy.is_max_radius {
            if !galaxy.is_max_radius {
                galaxy.is_max_radius = true;
            }
            if galaxy.diameter < 0.1 {
                galaxy.is_max_radius = false;
            }
            galaxy.theta -= galaxy.theta_step;
            galaxy.diameter -= 0.1;
        }

        if !galaxy.is_max_radius {
            galaxy.theta += galaxy.theta_step;
            galaxy.diameter += 0.1;
        }

        let hx = galaxy.hptr_x;
        let hy = galaxy.hptr_y;
        let q = (hx / hy - 1.0) * galaxy.theta; // create hypotrochoid

        unsafe{
            let curvature = curv_step as f64 / galaxy.curvature as f64;
            let h_delta = hx - hy;
            let cur_x = h_delta * galaxy.theta.cos() + galaxy.diameter * q.cos() + (ORIG_X + (galaxy.x - ORIG_X) * curvature) - h_delta;
            let cur_y = h_delta * galaxy.theta.sin() - galaxy.diameter * q.sin() + (ORIG_Y + (galaxy.y - ORIG_Y) * curvature);

            if prev_x != 0.0 {
                draw_line(hdc, (prev_x as i32, prev_y as i32), (cur_x as i32, cur_y as i32), galaxy.color);
            }

            prev_x = cur_x;
            prev_y = cur_y;
        }
    }
    unsafe {
        ORIG_X = galaxy.x;
        ORIG_Y = galaxy.y;
    };
}

fn draw_galaxy_step(hdc: HDC, galaxy: &mut Galaxy) {
    let mut prev_x = 0.0;
    let mut prev_y = 0.0;
    for curv_step in (0..galaxy.curvature).rev() {
        if galaxy.diameter > galaxy.max_diameter || galaxy.is_max_radius {
            if !galaxy.is_max_radius {
                galaxy.is_max_radius = true;
            }
            if galaxy.diameter < 0.1 {
                galaxy.is_max_radius = false;
            }
            galaxy.theta -= galaxy.theta_step;
            galaxy.diameter -= 0.1;
        }

        if !galaxy.is_max_radius {
            galaxy.theta += galaxy.theta_step;
            galaxy.diameter += 0.1;
        }

        let hx = galaxy.hptr_x;
        let hy = galaxy.hptr_y;
        let q = (hx / hy - 1.0) * galaxy.theta; // create hypotrochoid

        unsafe{
            let curvature = curv_step as f64 / galaxy.curvature as f64;
            let h_delta = hx - hy;
            let cur_x = h_delta * galaxy.theta.cos() + galaxy.diameter * q.cos() + (ORIG_X + (galaxy.x - ORIG_X) * curvature) - h_delta;
            let cur_y = h_delta * galaxy.theta.sin() - galaxy.diameter * q.sin() + (ORIG_Y + (galaxy.y - ORIG_Y) * curvature);

            if prev_x != 0.0 {
                draw_line(hdc, (prev_x as i32, prev_y as i32), (cur_x as i32, cur_y as i32), galaxy.color);
            }

            prev_x = cur_x;
            prev_y = cur_y;
        }
    }
    unsafe {
        ORIG_X = galaxy.x;
        ORIG_Y = galaxy.y;
    };
}

fn draw_line(hdc: HDC, from: (i32, i32), to: (i32, i32), color: u32) {
    let pen = unsafe { CreatePen(PS_SOLID as i32, 2, color) };
    let old_pen = unsafe { SelectObject(hdc, pen as _) };
    unsafe { MoveToEx(hdc, from.0, from.1, null_mut()) };
    unsafe { LineTo(hdc, to.0, to.1) };
    unsafe { SelectObject(hdc, old_pen) };
    unsafe { DeleteObject(pen as _) };
}










// MONDERBROTE

//extern crate minifb;
//
//use minifb::{Key, Window, WindowOptions};
//
//const WIDTH: usize = 800;
//const HEIGHT: usize = 800;
//
//const MAX_ITER: u32 = 100;
//const ZOOM_FACTOR: f64 = 1.1;
//
//fn mandelbrot(cx: f64, cy: f64, zoom: f64) -> u32 {
//    let mut x = 0.0;
//    let mut y = 0.0;
//    let mut i = 0;
//
//    let adjusted_cx = cx / zoom;
//    let adjusted_cy = cy / zoom;
//
//    while x * x + y * y <= 4.0 && i < MAX_ITER {
//        let x_temp = x * x - y * y + adjusted_cx;
//        y = 2.0 * x * y + adjusted_cy;
//        x = x_temp;
//        i += 1;
//    }
//
//    i
//}
//
//fn main() {
//    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
//
//    let mut zoom_level = 1.0;
//    let mut center_x = 0.0;
//    let mut center_y = 0.0;
//
//    let mut window = Window::new(
//        "Mandelbrot Set - Rust",
//        WIDTH,
//        HEIGHT,
//        WindowOptions::default(),
//    )
//    .unwrap_or_else(|e| {
//        panic!("{}", e);
//    });
//
//    while window.is_open() && !window.is_key_down(Key::Escape) {
////        if window.is_key_down(Key::Up) {
////            center_y -= 0.1 / zoom_level;
////        }
////        if window.is_key_down(Key::Down) {
////            center_y += 0.1 / zoom_level;
////        }
////        if window.is_key_down(Key::Left) {
////            center_x -= 0.1 / zoom_level;
////        }
////        if window.is_key_down(Key::Right) {
////            center_x += 0.1 / zoom_level;
////        }
////        if window.is_key_down(Key::Space) {
////            zoom_level *= ZOOM_FACTOR;
////        }
////        if window.is_key_down(Key::LeftCtrl) {
////            zoom_level /= ZOOM_FACTOR;
////        }
//
//        zoom_level += 0.01_f64;
//        use std::time::Instant;
//        let now = Instant::now();
//        for y in 0..HEIGHT {
//            for x in 0..WIDTH {
//                let cx = (x as f64 - WIDTH as f64 / 2.0) * 4.0 / (WIDTH as f64 * zoom_level) + center_x;
//                let cy = (y as f64 - HEIGHT as f64 / 2.0) * 4.0 / (HEIGHT as f64 * zoom_level) + center_y;
//
//                let color_value = mandelbrot(cx, cy, zoom_level) % 256;
//                let color = (color_value << 16) | (color_value << 8) | color_value;
//
//                buffer[y * WIDTH + x] = color;
//            }
//        }
//        let elapsed = now.elapsed();
//        println!("Elapsed: {:.2?}", elapsed);
//
//        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
//    }
//}
