use std::ops::Range;

use live_wallpapers::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use once_cell::sync::Lazy;
use rapier2d::prelude::{RigidBodySet, ColliderSet};

use super::{LiveData, physics::{PhysicsData, create_pipeline, create_edges}, bacteries::Bacteries};

pub static mut APP_DATA : Lazy::<AppData> = AppData::lazy();

/// Ignore DPI.
pub struct AppData {
    pub width: usize,
    pub height: usize,
    pub frame_num: u128,
    pub frame_processed: bool,
    pub delta_time: f32,
    pub bg_progress: u128,
    pub live_data: LiveData,
    pub physics_data: PhysicsData,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            width: unsafe { GetSystemMetrics(SM_CXSCREEN) } as usize,
            height: unsafe { GetSystemMetrics(SM_CYSCREEN) } as usize,
            frame_num: 0,
            frame_processed: false,
            delta_time: 0.016666,
            bg_progress: 0,
            live_data: LiveData::default(),
            physics_data: Default::default(),
        }
    }

    pub const fn lazy() -> Lazy<AppData> {
        Lazy::new(|| { AppData::new() })
    }
    
    pub fn build_physics(&mut self) {
        let rigidbody_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        
        let physics_data = create_pipeline(rigidbody_set, collider_set);
        self.physics_data = physics_data;
    }
    
    pub fn spawn_bacteries(&mut self, radius: Range<i32>) {
        self.live_data.bacteries = Bacteries::rand_in_rect(300, 0.0, self.width as f32, 0.0, self.height as f32);
        self.live_data.bacteries.set_random_radius(radius.start, radius.end);
        self.live_data.bacteries.actualize_rigidbodies(&mut self.physics_data.bodies);
        self.live_data.bacteries.actualize_colliders(&mut self.physics_data.colliders, &mut self.physics_data.bodies);
    }
    
    pub fn with_edges(&mut self, edge_width: f32, edge_height: f32) {
        create_edges(self.width as f32, self.height as f32, edge_width, edge_height,
            &mut self.physics_data.bodies,
            &mut self.physics_data.colliders);
    }
}

pub fn ref_app_data() -> &'static AppData {
    unsafe { &APP_DATA }
}

pub fn mut_app_data() -> &'static mut AppData {
    unsafe { &mut APP_DATA }
}