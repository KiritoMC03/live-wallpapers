use std::{ops::Range, sync::Mutex};

use live_wallpapers::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use once_cell::sync::Lazy;
use rapier2d::prelude::{RigidBodySet, ColliderSet};

use super::{LiveData, physics::{create_pipeline, create_edges}, bacteries::Bacteries};

pub static mut APP_DATA : Lazy::<Mutex<AppData>> = AppData::lazy();

/// Ignore DPI.
pub struct AppData {
    pub width: usize,
    pub height: usize,
    pub frame_num: u128,
    pub frame_processed: bool,
    pub frames_in_day: f32,
    pub day_progress: f32,
    pub delta_time: f32,
    pub live_data: LiveData,
}

impl AppData {
    pub fn new() -> AppData {
        const DELTA_TIME: f32 = 0.016666;
        AppData {
            width: unsafe { GetSystemMetrics(SM_CXSCREEN) } as usize,
            height: unsafe { GetSystemMetrics(SM_CYSCREEN) } as usize,
            frame_num: 0,
            frame_processed: false,
            delta_time: DELTA_TIME,
            frames_in_day: 8.0 * 60.0 / DELTA_TIME,
            day_progress: 0.5,
            live_data: LiveData::default(),
        }
    }

    pub const fn lazy() -> Lazy<Mutex<AppData>> {
        Lazy::new(|| { Mutex::new(AppData::new()) })
    }
    
    pub fn build_physics(&mut self) {
        let rigidbody_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        
        let physics_data = create_pipeline(rigidbody_set, collider_set);
        self.live_data.physics_data = physics_data;
    }
    
    pub fn spawn_bacteries(&mut self, radius: Range<i32>) {
        let settings = &self.live_data.settings;
        self.live_data.bacteries = Bacteries::rand_in_rect(200, 1000, 0.0..self.width as f32, 0.0..self.height as f32, settings.start_alive_range.clone());
        let bac = &mut self.live_data.bacteries;
        bac.set_random_radius(radius.start, radius.end);
        bac.actualize_rigidbodies(&mut self.live_data.physics_data.bodies, self.live_data.settings.dead_time);
        bac.actualize_colliders(&mut self.live_data.physics_data.colliders, &mut self.live_data.physics_data.bodies);
    }
    
    pub fn with_edges(&mut self, edge_width: f32, edge_height: f32) {
        create_edges(self.width as f32, self.height as f32, edge_width, edge_height,
            &mut self.live_data.physics_data.bodies,
            &mut self.live_data.physics_data.colliders);
    }
}

pub fn ref_app_data() -> &'static Mutex<AppData> {
    unsafe { &APP_DATA }
}

pub fn mut_app_data() -> &'static mut Mutex<AppData> {
    unsafe { &mut APP_DATA }
}