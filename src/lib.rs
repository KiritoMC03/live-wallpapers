/*
pub type c_uint = u32;
pub type c_int = i32;
pub type wchar_t = u16;
pub type c_ushort = u16;
pub type c_ulong = u32;

type BOOL = c_int;

pub type UINT = c_uint;
pub type PVOID = *mut core::ffi::c_void;
pub type LPVOID = *mut core::ffi::c_void;
pub type HANDLE = PVOID;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HANDLE;
pub type HBRUSH = HANDLE;
pub type HWND = HANDLE;
pub type HICON = HANDLE;
pub type HMENU = HANDLE;
pub type HCURSOR = HICON;

pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;

pub type UINT_PTR = usize;
pub type WPARAM = UINT_PTR;

pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;

pub type WORD = c_ushort;
pub type ATOM = WORD;

pub type DWORD = c_ulong;

pub type WNDPROC = Option<
  unsafe extern "system" fn(
      hwnd: HWND,
      uMsg: UINT,
      wParam: WPARAM,
      lParam: LPARAM,
      ) -> LRESULT,
>;
*/

use std::os::raw::c_int;
use std::default::Default;
pub use core::ptr::{null, null_mut};

pub use winapi::shared::ntdef::LPCWSTR;

pub use winapi::um::errhandlingapi::GetLastError;
pub use winapi::um::libloaderapi::GetModuleHandleW;

pub use winapi::um::winuser::{
    WNDCLASSW,
    WNDPROC,
    PAINTSTRUCT,
    MSG,
    WM_PAINT,
    COLOR_WINDOW,
    IDC_ARROW,
    DefWindowProcW,
    RegisterClassW,
    CreateWindowExW,
    ShowWindow,
    DestroyWindow,
    PostQuitMessage,
    GetMessageW,
    TranslateMessage,
    DispatchMessageW,
    LoadCursorW
};

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
    wc.lpfnWndProc = Some(DefWindowProcW);
    wc.hInstance = h_instance;
    wc.lpszClassName = name.as_ptr();
    wc.hCursor = unsafe { LoadCursorW(h_instance, IDC_ARROW) };
    (wc, h_instance)
}

pub fn create_window_handle(wc: &WNDCLASSW, wc_name: &Vec<u16>, window_name: &Vec<u16>, h_instance: HINSTANCE, ) -> HWND {
    let atom = unsafe { RegisterClassW(wc) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

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
            core::ptr::null_mut(),
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
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    
    0isize
}