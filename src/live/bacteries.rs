use std::{f32::consts::PI, ops::Range};

use micromath::vector::F32x2;
use rapier2d::prelude::*;

use super::{genome::Genome, utils::{rand_ranged_f32, rand_range_vec2, rand_ranged_i32}};

pub struct Collision {
    pub a: usize,
    pub b: usize,
}

#[derive(Default)]
pub struct Bacteries {
    pub num: usize,
    pub pos: Vec<F32x2>,
    pub radius: Vec<i32>,
    pub left_time: Vec<f32>,
    pub energy: Vec<f32>,
    pub parent: Vec<usize>,
    pub is_parented: Vec<bool>,
    pub rigidbody: Vec<RigidBodyHandle>,
    pub collider: Vec<ColliderHandle>,
    pub genome: Genome,
}



impl Bacteries {
    #[inline(always)]
    pub fn new(num: usize) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![F32x2::default(); num],
            radius: vec![i32::default(); num],
            left_time: vec![0.0; num],
            energy: vec![0.0; num],
            parent: vec![0; num],
            is_parented: vec![false; num],

            rigidbody: Vec::with_capacity(num),
            collider: Vec::with_capacity(num),

            genome: Genome::new(num),
        };

        result
    }

    #[inline(always)]
    pub const fn empty() -> Bacteries {
        let result = Bacteries {
            num: 0,
            pos: vec![],
            radius: vec![],
            left_time: vec![],
            energy: vec![],
            parent: vec![],
            is_parented: vec![],

            rigidbody: vec![],
            collider: vec![],

            genome: Genome::empty(),
        };

        result
    }

    #[inline(always)]
    pub fn rand_in_rect(num: usize, capacity: usize, x: Range::<f32>, y: Range::<f32>, start_alive_range: Range<f32>) -> Bacteries {
        let mut result = Bacteries::new(capacity);

        for i in 0..num {
            result.left_time[i] = rand_ranged_f32(start_alive_range.clone());
            result.pos.push(rand_range_vec2(x.clone(), y.clone()));
        }

        result
    }

    #[inline(always)]
    pub fn set_random_radius(&mut self, min: i32, max: i32) {
        self.radius = Vec::with_capacity(self.num);
        for _ in self.into_iter() {
            self.radius.push(rand_ranged_i32(min..max))
        }
    }

    #[inline(always)]
    pub fn actualize_rigidbodies(&mut self, rigidbody_set: &mut RigidBodySet, dead_time: f32) {
        while self.rigidbody.len() < self.num {
            let pos = self.pos[self.rigidbody.len()];
            let rb = RigidBodyBuilder::dynamic()
                .enabled(self.is_alive(self.rigidbody.len(), dead_time))
                .position(Isometry::new(vector![pos.x, pos.y], 0.0))
                .build();
            self.rigidbody.push(rigidbody_set.insert(rb));
        }
    }

    #[inline(always)]
    pub fn actualize_colliders(&mut self, colliders_set: &mut ColliderSet, rigidbody_set: &mut RigidBodySet) {
        for i in self.into_iter() {
            if self.collider.len() > i {
                colliders_set.get_mut(self.collider[i]).unwrap().shape_mut().as_ball_mut().unwrap().radius = self.radius[i] as f32;
            }
            else {
                let radius = self.radius[i] as f32;
                let collider = ColliderBuilder::ball(radius)
                    .mass(4.0/3.0 * PI * radius * radius)
                    .user_data(i as u128)
                    .active_events(ActiveEvents::COLLISION_EVENTS)
                    .build();
                let rb = self.rigidbody[i];
                self.collider.push(colliders_set.insert_with_parent(collider, rb, rigidbody_set));
            }
        }
    }

    #[inline(always)]
    pub fn draw<T: Fn(F32x2, i32) -> ()>(&self, draw_func: T) {
        for i in self.into_iter() {
            draw_func(self.pos[i], self.radius[i]);
        }
    }

    #[inline(always)]
    pub fn clamp_pos(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        for i in self.into_iter() {
            let mut cur = self.pos[i];
            cur.x = cur.x.clamp(x_min, x_max);
            cur.y = cur.y.clamp(y_min, y_max);
            self.pos[i] = cur;
        }
    }

    #[inline(always)]
    pub fn is_dead(&self, idx: usize, dead_time: f32) -> bool {
        self.left_time[idx] <= dead_time
    }

    #[inline(always)]
    pub fn is_alive(&self, idx: usize, dead_time: f32) -> bool {
        self.left_time[idx] > dead_time
    }

    #[inline(always)]
    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.num
    }
}