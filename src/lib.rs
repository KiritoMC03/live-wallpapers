use std::os::raw::c_int;
use std::default::Default;
pub use core::ptr::{null, null_mut};

pub use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::BOOL;
pub use winapi::shared::ntdef::LPCWSTR;

pub use winapi::um::errhandlingapi::GetLastError;
pub use winapi::um::libloaderapi::GetModuleHandleW;

pub use winapi::um::winuser::{
    WNDCLASSW,
    WNDPROC,
    PAINTSTRUCT,
    CREATESTRUCTW,
    MSG,
    WM_PAINT,
    COLOR_WINDOW,
    IDC_ARROW,
    GWLP_USERDATA,
    SMTO_NORMAL,
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    ShowWindow,
    EnumWindows,
    DestroyWindow,
    PostQuitMessage,
    FindWindowExW,
    LoadCursorW,
    SetCursor,
    GetWindowLongPtrW,
    SetWindowLongPtrW,
    SendMessageTimeoutW,
    SetParent,

    GetClientRect,
    GetDC,
    ReleaseDC,
};

use winapi::um::winuser::{GetMessageW, TranslateMessage, DispatchMessageW, WM_NCCREATE, WM_CREATE, WM_SETCURSOR, COLOR_BACKGROUND, COLOR_HIGHLIGHT, };
use winapi::um::winuser::{BeginPaint, FillRect, EndPaint};

pub use winapi::shared::windef::{
    HWND,
    HICON,
    HCURSOR,
    HBRUSH,
    RECT,
    HDC,
    POINT,
};

pub use winapi::shared::minwindef::{
    UINT,
    BYTE,
    HINSTANCE,
    WPARAM,
    LPARAM,
    LRESULT,
};

pub use winapi::um::winuser::{
    SW_SHOW,

    WS_OVERLAPPED,
    WS_CAPTION,
    WS_SYSMENU,
    WS_THICKFRAME,
    WS_MINIMIZEBOX,
    WS_MAXIMIZEBOX,
    WS_OVERLAPPEDWINDOW,

    CW_USEDEFAULT,

    WM_CLOSE,
    WM_DESTROY,
};

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
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
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

pub static mut WORKER_W : HWND = null_mut();

pub fn create_window(handle: HWND) {
    let _previously_visible = unsafe { ShowWindow(handle, SW_SHOW) };

    let r1 = unsafe { SendMessageTimeoutW(handle, 0x052C, 0, 0, SMTO_NORMAL, 1000, null_mut()) };
    let r2 = unsafe { SendMessageTimeoutW(handle, 0x052C, 0x0d, 0, SMTO_NORMAL, 1000, null_mut()) };
    let r3 = unsafe { SendMessageTimeoutW(handle, 0x052C, 0x0d, 1, SMTO_NORMAL, 1000, null_mut()) };

    println!("r {}", r1);
    println!("r {}", r2);
    println!("r {}", r3);

//    let wallpaper_hwnd : HWND = null_mut();
    unsafe { EnumWindows(Some(enum_windows_proc), 0) };
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let a = wide_null("SHELLDLL_DefView");
    let p = unsafe { FindWindowExW(hwnd, null_mut(), a.as_ptr(), null_mut()) };

    if !p.is_null()
    {
        println!("found window!");
        let b = wide_null("WorkerW");
        // Gets the WorkerW Window after the current one.
        unsafe { WORKER_W = FindWindowExW(null_mut(), hwnd, b.as_ptr(), null_mut()) };
        if !WORKER_W.is_null(){
            println!("found WorkerW!");
        }

        SetParent(hwnd, WORKER_W);
    }

    return 1;
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
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

pub unsafe extern "system" fn window_procedure(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,) -> LRESULT {
    match Msg {
        WM_NCCREATE => {
            println!("NC Create");
            let createstruct: *mut CREATESTRUCTW = lParam as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr = (*createstruct).lpCreateParams;
            SetWindowLongPtrW(hWnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
            return 1;
        }
        WM_CREATE => println!("Create"),
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => {
            let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
            drop(Box::from_raw(ptr));
            println!("Cleaned up the box.");
            PostQuitMessage(0);
        }
        WM_PAINT => {
            println!("Paint !!!");
//            let mut ps : PAINTSTRUCT = PAINTSTRUCT::default();
//            let hdc : HDC = BeginPaint(hWnd, &mut ps);
//            let mut rect : RECT = RECT { left: 150, top: 170, right: 1000, bottom: 1000 };
//            GetClientRect(hWnd, &mut rect);
//            let brush = COLOR_HIGHLIGHT + 1;
//            FillRect(hdc, &rect, brush as HBRUSH);
//
//            let hdcDesktop = GetDC(null_mut());
//
//            ReleaseDC(null_mut(), hdcDesktop);
//
//            EndPaint(hWnd, &ps);
//            return 0;



            let ptr = GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *mut i32;
            println!("Current ptr: {}", *ptr);
            *ptr += 1;
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hWnd, &mut ps);
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_HIGHLIGHT + 1) as HBRUSH);
            EndPaint(hWnd, &ps);
        }
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    
    0
}