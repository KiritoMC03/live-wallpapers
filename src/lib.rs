pub use std::default::Default;
pub use core::ptr::null_mut;

pub use winapi::ctypes::c_int;
pub use winapi::shared::basetsd::LONG_PTR;
pub use winapi::shared::minwindef::BOOL;

pub use winapi::um::errhandlingapi::GetLastError;
pub use winapi::um::libloaderapi::GetModuleHandleW;

pub use winapi::um::wingdi::{RGB, CreateSolidBrush, GetRValue, GetBValue, GetGValue};
pub use winapi::um::winuser::{
    WNDCLASSW,
    WNDPROC,
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

pub use winapi::um::winuser::{
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

pub use winapi::um::winuser::{
    GetMessageW,
    TranslateMessage,
    DispatchMessageW,
    SystemParametersInfoW,
};

pub use winapi::um::winuser::{
    BeginPaint,
    FillRect,
    EndPaint,
};

pub use winapi::shared::windef::{
    HWND,
    HBRUSH,
};

pub use winapi::shared::minwindef::{
    UINT,
    HINSTANCE,
    WPARAM,
    LPARAM,
    LRESULT,
};

pub use winapi::um::winuser::{
    SW_SHOW,

    WM_CLOSE,
    WM_DESTROY,
};

pub const SHELLDLL_DEF_VIEW_STR : &str = "SHELLDLL_DefView";
pub const WORKER_W_STR : &str = "WorkerW";

static mut WORKER_W : HWND = null_mut();

pub fn create_window_class(name: &Vec<u16>, window_procedure: WNDPROC) -> (WNDCLASSW, HINSTANCE) {
    let h_instance = unsafe { GetModuleHandleW(core::ptr::null()) };

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = window_procedure;
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

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}