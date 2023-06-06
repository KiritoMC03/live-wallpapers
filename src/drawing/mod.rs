use winapi::um::wingdi::{
    SetPixel,
    CreateSolidBrush,
    DeleteObject,
};

use winapi::um::winuser::{
    PAINTSTRUCT,
    FillRect,
};

use winapi::shared::windef::{
    HDC,
    COLORREF,
};
  
pub fn draw_circle_brush(center_x: i32, center_y: i32, radius: i32, hdc: HDC, color: u32) {
    for x in -radius..radius {
        for y in -radius..radius {
            let sqr_distance = (x * x + y * y) as f64;
            if sqr_distance <= (radius * radius) as f64 {
                unsafe { SetPixel(hdc, center_x + x, center_y + y, color) };
            }
        }
    }
}

pub fn draw_fullscreen_rect(hdc: HDC, ps: &PAINTSTRUCT, color: COLORREF) {
    let rect = &ps.rcPaint;
    unsafe {
        let brush = CreateSolidBrush(color);
        FillRect(hdc, rect, brush);
        DeleteObject(brush as _);
    }
}