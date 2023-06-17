use std::f32::consts::PI;

use winapi::shared::windef::HDC;
use winapi::um::winuser::{RedrawWindow, RDW_INVALIDATE};
use super::app::AppData;
use super::bacteries::Bacteries;
use super::bacteries_processing::{FLAGELLA_NUM_RANGE, FLAGELLA_LEN_RANGE};

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
    close_draw_frame, draw_line, create_solid_pen, close_draw_lines
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


    // ToDo::::: move!!!
    for i in app.live_data.bacteries.into_iter() {
        let body = app.live_data.physics_data.get_rb(app.live_data.bacteries.rigidbody[i]);
        let pos = body.position();
        app.live_data.bacteries.pos[i].x = pos.translation.x;
        app.live_data.bacteries.pos[i].y = pos.translation.y;
    }

    paint_bacteries(frame.hdc, app);
    close_draw_frame(hdc, app.width as i32, app.height as i32, frame);
}

fn paint_bacteries(hdc: HDC, app: &mut AppData) {
    let bac = &app.live_data.bacteries;

    let colors = [
        RGB::new(10, 180, 10),
        RGB::new(180, 10, 10),
        ];

//    let flagella_col = winapi::um::wingdi::RGB(0, 0, 0);
//    let draw_lines_data = create_solid_pen(hdc, flagella_col);
//    paint_flagella(hdc, &bac);
//    close_draw_lines(draw_lines_data);

    let a = std::time::Instant::now();
    for i in bac.into_iter() {
        if bac.is_alive(i) {
            let val = (0.5 - bac.genome.photosynth[i] / 2.0 + bac.genome.carnivore[i] / 2.0).clamp(0.0, 9.9);
            let col = interpolate_colors(&colors, val);
            let (brush, old_brush) = change_solid_brush(hdc, col);
            let pos = bac.pos[i];
            draw_circle(hdc, pos.x as i32, pos.y as i32, bac.radius[i]);
            revert_brush(hdc, brush, old_brush);
        }
    }
    println!("bacteries: {}", a.elapsed().as_millis());
}

#[inline(always)]
fn paint_flagella(hdc: HDC, bac: &Bacteries) {
    type Point = winapi::shared::windef::POINT;
    let mut pts = Vec::with_capacity(1005 + FLAGELLA_NUM_RANGE.end as usize);
    let mut poly_points = Vec::with_capacity(505 + FLAGELLA_NUM_RANGE.end as usize / 2);
    let mut total_num_flagella = 0u32;

    for i in bac.into_iter() {
        if bac.is_alive(i) {
            let len = (FLAGELLA_LEN_RANGE.start as f32 + (FLAGELLA_LEN_RANGE.end - FLAGELLA_LEN_RANGE.start) as f32 * bac.genome.movement_force[i]).round();
            let mut num_flagella = (FLAGELLA_NUM_RANGE.start as f32 + (FLAGELLA_NUM_RANGE.end - FLAGELLA_NUM_RANGE.start) as f32 * bac.genome.movement_rate[i]).round() as u32;
            if num_flagella % 2 == 1 && num_flagella > 0 {
                num_flagella -= 1;
            }

            if num_flagella == 0 {
                continue;
            }

            num_flagella /= 2;
            total_num_flagella += num_flagella as u32;

            let r = bac.radius[i] as f32 + len;
            let c = bac.pos[i];

            for i in 0..num_flagella as i32 {
                let angle = 2.0 * PI * (i as f32) / num_flagella as f32;
                let back_angle = angle + PI;
                let x1 = (c.x + r * angle.cos()) as i32;
                let y1 = (c.y + r * angle.sin()) as i32;
                let x2 = (c.x + r * back_angle.cos()) as i32;
                let y2 = (c.y + r * back_angle.sin()) as i32;
                pts.push(Point { x: x1, y: y1 });
                pts.push(Point { x: x2, y: y2 });
                poly_points.push(2);
            }

            if total_num_flagella >= 200 {
                paint(hdc, &pts, &poly_points, total_num_flagella);

                pts.clear();
                poly_points.clear();
                total_num_flagella = 0u32;
            }
        }
    }

    paint(hdc, &pts, &poly_points, total_num_flagella);

    fn paint(hdc: HDC, pts: &Vec<Point>, poly_points: &Vec<u32>, total_num_flagella: u32) {
        unsafe { winapi::um::wingdi::PolyPolyline(hdc, pts.as_ptr(), poly_points.as_ptr(), total_num_flagella) };
    }
}