use std::ops::Range;

use micromath::vector::F32x2;

use self::{physics::PhysicsData, utils::{rand_ranged_f32, rand_range_vec2}};
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
    pub settings: LiveSettings,
}

#[derive(Default, Debug)]
pub struct LiveSettings {
    pub move_force : f32,
    pub vel_range : Range<f32>,

    pub radius_range : Range<i32>,

    pub max_alive : f32,
    pub start_alive_range : Range<f32>,
    pub dead_time : f32,

    pub start_energy : f32,
    pub division_energy : f32,
    pub alive_to_energy_coef : f32,

    pub photosynth_rate : f32,
    pub carnivore_rate : f32,
    pub carnivore_damage : f32,
    pub defence : f32,
    pub carnivore_cost : f32,

    pub genome_mut_range : Range<f32>,
    pub radius_mut_range : Range<f32>,

    pub flagella_num_range : Range<i32>,
    pub flagella_len_range : Range<i32>,

    pub max_energy_distribution : f32,

    pub max_repulsive_force : f32,
}

impl LiveData {
    pub fn spawn_bac(&mut self, pos: F32x2, radius: i32) {
        for i in self.bacteries.into_iter() {
            if self.bacteries.is_dead(i, self.settings.dead_time) {
                self.bacteries.pos[i] = pos;
                self.bacteries.radius[i] = radius;
                self.bacteries.left_time[i] = rand_ranged_f32(self.settings.start_alive_range.clone());
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
            if self.bacteries.is_dead(i, self.settings.dead_time) {
                let pos = self.bacteries.pos[src];
                let mut radius = (self.bacteries.radius[src] as f32 * rand_ranged_f32(self.settings.radius_mut_range.clone())) as i32;
                radius = radius.clamp(self.settings.radius_range.start, self.settings.radius_range.end);
                self.bacteries.pos[i] = pos + rand_range_vec2(-0.1..0.1, -0.1..0.1);
                self.bacteries.radius[i] = radius;
                self.bacteries.left_time[i] = rand_ranged_f32(self.settings.start_alive_range.clone());
                self.bacteries.parent[i] = src;
                self.bacteries.is_parented[i] = true;
                self.bacteries.genome.mut_clone(src, i, self.settings.genome_mut_range.clone());

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

impl LiveSettings {
    pub fn new() -> LiveSettings {
        LiveSettings {
            move_force: 100.0,
            vel_range: -1.0..1.0,
            radius_range: 8..20,
            max_alive: 100.0,
            start_alive_range: 1.0..100.0,
            dead_time: 0.0,
            start_energy: 1.0,
            division_energy: 10.0,
            alive_to_energy_coef: 0.1,
            photosynth_rate: 0.02,
            carnivore_rate: 10.0,
            carnivore_damage: 15.0,
            defence: 15.0,
            carnivore_cost: 20.0,
            genome_mut_range: 0.9..1.1,
            radius_mut_range: 0.9..1.1,
            flagella_num_range: 6..14,
            flagella_len_range: 2..8,
            max_energy_distribution: 10.0,
            max_repulsive_force: 300.0,
        }
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