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

use std::default::Default;
use core::ptr::{null, null_mut};

const SW_SHOW: c_int = 5;
const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
                                 | WS_CAPTION
                                 | WS_SYSMENU
                                 | WS_THICKFRAME
                                 | WS_MINIMIZEBOX
                                 | WS_MAXIMIZEBOX;
const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;



#[repr(C)]
pub struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}

impl WNDCLASSW {
    pub fn create(name: &Vec<u16>) -> (WNDCLASSW, HINSTANCE) {
        let h_instance = unsafe { GetModuleHandleW(core::ptr::null()) };

        let mut wc = WNDCLASSW::default();
        wc.lpfnWndProc = Some(DefWindowProcW);
        wc.hInstance = h_instance;
        wc.lpszClassName = name.as_ptr();        
        (wc, h_instance)
    }
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

impl Default for WNDCLASSW {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

#[link(name = "User32")]
extern "system" {
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
      dwExStyle: DWORD, lpClassName: LPCWSTR, lpWindowName: LPCWSTR,
      dwStyle: DWORD, X: c_int, Y: c_int, nWidth: c_int, nHeight: c_int,
      hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID,
      ) -> HWND;

    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    
    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(
        hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
        ) -> LRESULT;
}