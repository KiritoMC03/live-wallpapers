use std::ptr::null_mut;

use winapi::um::wingdi::{
    SelectObject,
    DeleteObject,
    SetPixel,
    CreateSolidBrush,
    CreatePen,
    MoveToEx,
    LineTo, CreateCompatibleDC, CreateCompatibleBitmap, BitBlt, SRCCOPY, DeleteDC,
};

use winapi::um::wingdi::PS_SOLID;

use winapi::um::winuser::{
    PAINTSTRUCT,
    FillRect,
};

use winapi::shared::windef::{
    HDC,
    COLORREF, HBITMAP,
};

pub struct DrawFrameData {
    pub hdc: HDC,
    h_bmp_mem: HBITMAP,
    h_old_bmp_mem: HBITMAP,
}

pub fn onep_draw_frame(hdc: HDC, width: i32, height: i32) -> DrawFrameData {
    unsafe {
        let h_mem_dc = CreateCompatibleDC(hdc);
        let h_bmp_mem = CreateCompatibleBitmap(hdc, width, height);
        let h_old_bmp_mem = SelectObject(h_mem_dc, h_bmp_mem as _) as HBITMAP;

        DrawFrameData { hdc: h_mem_dc, h_bmp_mem, h_old_bmp_mem }
    }
}

pub fn close_draw_frame(hdc: HDC, width: i32, height: i32, draw_frame_data: DrawFrameData) {
    unsafe {
        BitBlt(hdc, 0, 0, width, height, draw_frame_data.hdc, 0, 0, SRCCOPY);

        SelectObject(draw_frame_data.hdc, draw_frame_data.h_old_bmp_mem as _);
        DeleteObject(draw_frame_data.h_bmp_mem as _);
        DeleteDC(draw_frame_data.hdc);
    }
}

pub fn draw_line(hdc: HDC, from: (i32, i32), to: (i32, i32), color: u32) {
    let pen = unsafe { CreatePen(PS_SOLID as i32, 2, color) };
    let old_pen = unsafe { SelectObject(hdc, pen as _) };
    unsafe { MoveToEx(hdc, from.0, from.1, null_mut()) };
    unsafe { LineTo(hdc, to.0, to.1) };
    unsafe { SelectObject(hdc, old_pen) };
    unsafe { DeleteObject(pen as _) };
}
  
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

    todo!("Add custom parameters!");
}