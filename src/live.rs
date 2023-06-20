use micromath::vector::F32x2;

use crate::live::{bacteries_processing::RADIUS_MUT_RANGE};

use self::{physics::PhysicsData, bacteries_processing::{RADIUS_RANGE, START_ALIVE_RANGE}, utils::{rand_ranged_f32, rand_range_vec2}};
use rapier2d::prelude::*;

pub mod app;
pub mod physics;
pub mod graphics;
pub mod bacteries;
pub mod genome;
pub mod bacteries_processing;
pub mod save_load;
pub mod utils;

#[derive(Default)]
pub struct LiveData {
    pub bacteries: bacteries::Bacteries,
    pub physics_data: PhysicsData,
}

impl LiveData {
    pub fn spawn_bac(&mut self, pos: F32x2, radius: i32) {
        for i in self.bacteries.into_iter() {
            if self.bacteries.is_dead(i) {
                self.bacteries.pos[i] = pos;
                self.bacteries.radius[i] = radius;
                self.bacteries.left_time[i] = rand_ranged_f32(START_ALIVE_RANGE);
                self.bacteries.genome.default_one(i);

                let rb = self.physics_data.get_rb_mut(self.bacteries.rigidbody[i]);
                rb.set_position(Isometry::new(vector![pos.x, pos.y], 0.0), true);
                rb.set_enabled(true);

                let coll = self.physics_data.get_coll_mut(self.bacteries.collider[i]);
                coll.shape_mut().as_ball_mut().unwrap().radius = radius as f32;
                coll.set_enabled(true);
                return;
            }
        }
    }

    pub fn mut_clone(&mut self, src: usize) {
        for i in self.bacteries.into_iter() {
            if self.bacteries.is_dead(i) {
                let pos = self.bacteries.pos[src];
                let mut radius = (self.bacteries.radius[src] as f32 * rand_ranged_f32(RADIUS_MUT_RANGE)) as i32;
                radius = radius.clamp(RADIUS_RANGE.start, RADIUS_RANGE.end);
                self.bacteries.pos[i] = pos + rand_range_vec2(-0.1..0.1, -0.1..0.1);
                self.bacteries.radius[i] = radius;
                self.bacteries.left_time[i] = rand_ranged_f32(START_ALIVE_RANGE);
                self.bacteries.parent[i] = src;
                self.bacteries.is_parented[i] = true;
                self.bacteries.genome.mut_clone(src, i);

                let rb = self.physics_data.get_rb_mut(self.bacteries.rigidbody[i]);
                rb.set_position(Isometry::new(vector![pos.x, pos.y], 0.0), true);
                rb.set_enabled(false);

                let coll = self.physics_data.get_coll_mut(self.bacteries.collider[i]);
                coll.shape_mut().as_ball_mut().unwrap().radius = radius as f32;
                coll.set_enabled(true);
                self.bacteries.genome.normilize_one(i);
                return;
            }
        }
    }

    pub fn kill_bac(&mut self, idx: usize) {
        self.physics_data.get_rb_mut(self.bacteries.rigidbody[idx]).set_enabled(false);
        self.physics_data.get_coll_mut(self.bacteries.collider[idx]).set_enabled(false);
    }
}

pub fn normalize_f32x2(v: &mut F32x2) {
    let len = (v.x * v.x + v.y * v.y).sqrt();
    if len > 0.0 {
        v.x = v.x / len;
        v.y = v.y / len;
    }
}

pub fn len_f32x2(v: &F32x2) -> f32 {
    (v.x * v.x + v.y * v.y).sqrt()
}