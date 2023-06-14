use winapi::shared::windef::HDC;
use winapi::um::winuser::{RedrawWindow, RDW_INVALIDATE};
use super::app::AppData;

use live_wallpapers::{PAINTSTRUCT, MSG, null_mut, HWND};
use live_wallpapers::drawing::colors::{
    RGB,
    interpolate_colors
};

use live_wallpapers::drawing::primitives::{
    open_draw_frame,
    draw_fullscreen_rect,
    change_solid_brush,
    draw_circle,
    revert_brush,
    close_draw_frame
};

pub struct GraphicsPipeline<T: Fn(MSG) -> bool> {
    messages_handler: T,
}

impl<T: Fn(MSG) -> bool> GraphicsPipeline<T> {
    pub fn new(messages_handler: T) -> GraphicsPipeline<T> {
        GraphicsPipeline {
            messages_handler,
        }
    }
    
    pub fn step(&self, msg: MSG, app: &AppData, window_handle: HWND) {
        if (self.messages_handler)(msg) { }
        else if !app.frame_processed {
            unsafe { RedrawWindow(window_handle, null_mut(), null_mut(), RDW_INVALIDATE); }
        }
    }
}

pub fn paint_frame(hdc: HDC, ps: &PAINTSTRUCT, app: &mut AppData) {
    let colors = [
        RGB::new(226, 239, 84),
        RGB::new(255, 92, 102),
        RGB::new(98, 72, 213),
        RGB::new(226, 239, 84),
        ];

    let color = interpolate_colors(&colors, (app.bg_progress % 200 as u128) as f32 / 200_f32);
    let bactery_color = winapi::um::wingdi::RGB(143, 0, 0);

    let frame = open_draw_frame(hdc, app.width as i32, app.height as i32);
    draw_fullscreen_rect(frame.hdc, &ps, color);


    for i in app.live_data.bacteries.into_iter() {
        let body = app.physics_data.bodies.get(app.live_data.bacteries.rigidbody[i]).unwrap();
        let pos = body.position();
        app.live_data.bacteries.pos[i].x = pos.translation.x;
        app.live_data.bacteries.pos[i].y = pos.translation.y;
    }

    let bac = &app.live_data.bacteries;
    let (brush, old_brush) = change_solid_brush(frame.hdc, bactery_color);
    for i in bac.into_iter() {
        let pos = bac.pos[i];
        draw_circle(frame.hdc, pos.x as i32, pos.y as i32, bac.radius[i]);
    }
    revert_brush(frame.hdc, brush, old_brush);
    close_draw_frame(hdc, app.width as i32, app.height as i32, frame);
}