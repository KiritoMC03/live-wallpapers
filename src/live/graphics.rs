use winapi::shared::windef::HDC;
use winapi::um::winuser::{RedrawWindow, RDW_INVALIDATE};
use super::app::AppData;
use super::bacteries::Bacteries;

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

    let frame = open_draw_frame(hdc, app.width as i32, app.height as i32);
    draw_fullscreen_rect(frame.hdc, &ps, color);


    for i in app.live_data.bacteries.into_iter() {
        let body = app.live_data.physics_data.get_rb(app.live_data.bacteries.rigidbody[i]);
        let pos = body.position();
        app.live_data.bacteries.pos[i].x = pos.translation.x;
        app.live_data.bacteries.pos[i].y = pos.translation.y;
    }

    let bac = &app.live_data.bacteries;

    let green = winapi::um::wingdi::RGB(10, 180, 10);
    let red = winapi::um::wingdi::RGB(180, 10, 10);

//    let (brush, old_brush) = change_solid_brush(frame.hdc, green);
//    for i in bac.into_iter() {
//        if bac.genome.photosynth[i] > bac.genome.carnivore[i] {
//            if bac.is_alive(i) {
//                let pos = bac.pos[i];
//                draw_circle(frame.hdc, pos.x as i32, pos.y as i32, bac.radius[i]);
//            }
//        }
//    }
//    revert_brush(frame.hdc, brush, old_brush);

    paint_bacteries(frame.hdc, bac, |i| { bac.genome.photosynth[*i] > bac.genome.carnivore[*i]  }, green);
    paint_bacteries(frame.hdc, bac, |i| { bac.genome.photosynth[*i] < bac.genome.carnivore[*i]  }, red);

    close_draw_frame(hdc, app.width as i32, app.height as i32, frame);
}

fn paint_bacteries<P: FnMut(&usize) -> bool>(hdc: HDC,
                                             bac: &Bacteries,
                                             filter: P,
                                             color: u32) {
    let (brush, old_brush) = change_solid_brush(hdc, color);
    for i in bac.into_iter().filter(filter) {
        if bac.is_alive(i) {
            let pos = bac.pos[i];
            draw_circle(hdc, pos.x as i32, pos.y as i32, bac.radius[i]);
        }
    }
    revert_brush(hdc, brush, old_brush);
}