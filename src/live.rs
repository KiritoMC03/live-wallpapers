use micromath::vector::F32x2;

use crate::live::{bacteries::rand_ranged_f32, bacteries_processing::RADIUS_MUT_RANGE};

use self::{physics::PhysicsData, bacteries_processing::ALIVE_TIME};
use rapier2d::prelude::*;

pub mod app;
pub mod physics;
pub mod graphics;
pub mod bacteries;
pub mod bacteries_processing;

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
                self.bacteries.left_time[i] = rand_ranged_f32(ALIVE_TIME);
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
                let radius = (self.bacteries.radius[src] as f32 * rand_ranged_f32(RADIUS_MUT_RANGE)) as i32;
                self.bacteries.pos[i] = pos;
                self.bacteries.radius[i] = radius;
                self.bacteries.left_time[i] = rand_ranged_f32(ALIVE_TIME);
                self.bacteries.genome.mut_clone(src, i);

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

    pub fn kill_bac(&mut self, idx: usize) {
        self.physics_data.get_rb_mut(self.bacteries.rigidbody[idx]).set_enabled(false);
        self.physics_data.get_coll_mut(self.bacteries.collider[idx]).set_enabled(false);
    }
}