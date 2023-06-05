use std::default::Default;
use core::ptr::null_mut;

use winapi::ctypes::c_int;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::BOOL;

use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;

use winapi::um::wingdi::{RGB, CreateSolidBrush, GetRValue, GetBValue, GetGValue};
use winapi::um::winuser::{
    WNDCLASSW,
    PAINTSTRUCT,
    CREATESTRUCTW,
    MSG,
    IDC_ARROW,
    GWLP_USERDATA,
    SMTO_NORMAL,

    WM_CREATE,
    WM_NCCREATE,
    WM_PAINT,
    WS_POPUP,
    WS_VISIBLE,
    SM_CXSCREEN,
    SM_CYSCREEN,
    SWP_NOZORDER,
    SWP_NOOWNERZORDER,

    COLOR_HIGHLIGHT, GCLP_HBRBACKGROUND, SetClassLongPtrA,
};

use winapi::um::winuser::{
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    ShowWindow,
    EnumWindows,
    DestroyWindow,
    PostQuitMessage,
    FindWindowW,
    FindWindowExW,
    GetWindowLongPtrW,
    SetWindowLongPtrW,
    SendMessageTimeoutW,

    GetSystemMetrics,
    SetWindowPos,
    SetParent,
    GetDesktopWindow,
    GetDpiForWindow,

    LoadCursorW,
};

use winapi::um::winuser::{
    GetMessageW,
    TranslateMessage,
    DispatchMessageW,
    SystemParametersInfoW,
};

use winapi::um::winuser::{
    BeginPaint,
    FillRect,
    EndPaint,
};

use winapi::shared::windef::{
    HWND,
    HBRUSH,
};

use winapi::shared::minwindef::{
    UINT,
    HINSTANCE,
    WPARAM,
    LPARAM,
    LRESULT,
};

use winapi::um::winuser::{
    SW_SHOW,

    WM_CLOSE,
    WM_DESTROY,
};

pub const SHELLDLL_DEF_VIEW_STR : &str = "SHELLDLL_DefView";
pub const WORKER_W_STR : &str = "WorkerW";

static mut WORKER_W : HWND = null_mut();

pub fn create_window_class(name: &Vec<u16>) -> (WNDCLASSW, HINSTANCE) {
    let h_instance = unsafe { GetModuleHandleW(core::ptr::null()) };

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = h_instance;
    wc.lpszClassName = name.as_ptr();
    wc.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    (wc, h_instance)
}

pub fn create_window_handle(wc: &WNDCLASSW, wc_name: &Vec<u16>, window_name: &Vec<u16>, h_instance: HINSTANCE, ) -> HWND {
    let atom = unsafe { RegisterClassW(wc) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

    let lparam: *mut i32 = Box::leak(Box::new(5_i32));
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            wc_name.as_ptr(),
            window_name.as_ptr(),
            WS_POPUP | WS_VISIBLE,
            0,
            0,
            0,
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            h_instance,
            lparam.cast(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }
    
    hwnd
}

pub fn create_window(handle: HWND) {
    let _previously_visible = unsafe { ShowWindow(handle, SW_SHOW) };
}

/// Find `Progman` and get handle
pub fn get_progman_handle() -> HWND {
    let h_progman = unsafe { FindWindowW(wide_null("Progman").as_ptr(), null_mut()) };
    h_progman
}

/// Message to `Progman` to spawn a `WorkerW`
pub fn try_spawn_worker_w(progman_handle: HWND) -> Result<(), &'static str> {
    // Requare all!
    let send_message_results = unsafe { [
        SendMessageTimeoutW(progman_handle, 0x052C, 0, 0, SMTO_NORMAL, 1000, null_mut()),
        SendMessageTimeoutW(progman_handle, 0x052C, 0x0d, 0, SMTO_NORMAL, 1000, null_mut()),
        SendMessageTimeoutW(progman_handle, 0x052C, 0x0d, 1, SMTO_NORMAL, 1000, null_mut())
    ] };

    if send_message_results.iter().all(|r| *r == 0) {
        return Err("`Progman` failed to spawn WorkerW!");
    }

    Ok(())
}

/// Find the newly created `WorkerW`
pub fn find_worker_w() -> HWND {
    unsafe {
        EnumWindows(Some(enum_windows_proc), 0);
        return WORKER_W.clone();
    };
}

/// Get desktop dpi and window dpi and return aspect
pub fn get_dpi_aspect(handle: HWND) -> f64 {
    let desktop_dpi = unsafe { GetDpiForWindow(GetDesktopWindow()) };
    let dpi_aspect = desktop_dpi as f64 / unsafe { GetDpiForWindow(handle) } as f64;
    dpi_aspect
}

pub fn pull_window_to_desktop(handle: HWND, worker_w_handle: HWND, dpi_aspect: f64) {
    unsafe { SetParent(handle, worker_w_handle) };
    unsafe {
        SetWindowPos(
            handle,
            null_mut(),
            0,
            0,
            (GetSystemMetrics(SM_CXSCREEN) as f64 * dpi_aspect) as c_int,
            (GetSystemMetrics(SM_CYSCREEN) as f64 * dpi_aspect) as c_int,
            SWP_NOOWNERZORDER | SWP_NOZORDER
        )
    };

    unsafe { SystemParametersInfoW(20, 0, null_mut(), 0x1) };
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let shelldll_def_view_name = wide_null(SHELLDLL_DEF_VIEW_STR);
    let cur_hwnd = unsafe { FindWindowExW(hwnd, null_mut(), shelldll_def_view_name.as_ptr(), null_mut()) };

    if !cur_hwnd.is_null()
    {
        println!("{} window found!", SHELLDLL_DEF_VIEW_STR);
        let worker_w_name = wide_null(WORKER_W_STR);
        // Gets the WorkerW Window after the current one.
        unsafe { WORKER_W = FindWindowExW(null_mut(), hwnd, worker_w_name.as_ptr(), null_mut()) };
        if !WORKER_W.is_null() {
            println!("{} window found!", WORKER_W_STR);
        }
    }

    return 1;
}

pub fn handle_window_messages(mut msg: MSG) {
    let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
    if message_return == 0 {
        return;
    } else if message_return == -1 {
        let last_error = unsafe { GetLastError() };
        panic!("Error with `GetMessageW`, error code: {}", last_error);
    } else {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

static mut FRAME : u128 = 0;

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
//            println!("Paint !!!");


            let colors = [
                CUSTOM_RGB::new(255, 0, 0),   // Red
                CUSTOM_RGB::new(0, 255, 0),   // Green
                CUSTOM_RGB::new(0, 0, 255),   // Blue
                CUSTOM_RGB::new(255, 255, 0), // Yellow
            ];


            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            let rect = &ps.rcPaint;

            // Fill the background with a specific color (e.g., blue)
            let color = interpolate_colors(&colors, (FRAME % 600 as u128) as f32 / 600_f32);
            println!("unpacked: {}-{}-{}", GetRValue(color), GetGValue(color) ,GetBValue(color));
            //            println!("col: {}, {}", color, FRAME);
            let hbrush = CreateSolidBrush(color);
            FillRect(hdc, rect, hbrush);

            EndPaint(hwnd, &ps);
            FRAME += 1;



//            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
//            println!("Current ptr: {}", *ptr);
//            *ptr += 1;
//            let mut ps = PAINTSTRUCT::default();
//            let hdc = BeginPaint(hwnd, &mut ps);
//
//            let brush_color = RGB(255, 0, 0); // Color is set to red (change as needed)
//            let _success = FillRect(hdc, &ps.rcPaint, (brush_color) as HBRUSH);
//            EndPaint(hwnd, &ps);
        }
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }
    
    0
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

type CUSTOM_RGB = rgb::RGB<u8>;

fn interpolate_colors(colors: &[CUSTOM_RGB], weight: f32) -> u32 {
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

    println!("{}-{}-{}", r, g ,b);
    RGB(r, g, b)
}