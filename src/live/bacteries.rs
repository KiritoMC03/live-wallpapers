use std::f32::consts::PI;

use micromath::vector::F32x2;
use rand::Rng;
use rapier2d::prelude::*;

pub struct Collision {
    pub a: usize,
    pub b: usize,
}

#[derive(Default)]
pub struct Bacteries {
    pub num: usize,
    pub pos: Vec<F32x2>,
    pub radius: Vec<i32>,
    pub rigidbody: Vec<RigidBodyHandle>,
    pub collider: Vec<ColliderHandle>,
    pub genome: Genome,
}

#[derive(Default, Clone)]
pub struct Genome {
    pub length: usize,
    pub photosynth: Vec<Gen>,
    pub movement_force: Vec<Gen>,
    pub movement_rate: Vec<Gen>,
}

pub type Gen = f32;

pub trait GenTrait {
    fn get(&self, idx: usize) -> Gen;
    fn fill_default(&mut self);
}

impl GenTrait for Vec<Gen> {
    #[inline(always)]
    fn get(&self, idx: usize) -> Gen {
        self[idx]
    }

    #[inline(always)]
    fn fill_default(&mut self) {
        for el in self {
            *el = rand_ranged_f32(0.0, 1.0);
        }
    }
}

impl Bacteries {
    pub fn new(num: usize) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![F32x2::default(); num],
            radius: vec![i32::default(); num],
            rigidbody: vec![RigidBodyHandle::default(); num],
            collider: vec![ColliderHandle::default(); num],
            genome: Genome::new(num),
        };

        result
    }

    pub const fn empty() -> Bacteries {
        let result = Bacteries {
            num: 0,
            pos: vec![],
            radius: vec![],
            rigidbody: vec![],
            collider: vec![],
            genome: Genome::empty(),
        };

        result
    }

    pub fn rand_in_rect(num: usize, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Bacteries {
        let mut result = Bacteries {
            num,
            pos: Vec::with_capacity(num),
            radius: vec![0; num],
            rigidbody: Vec::with_capacity(num),
            collider: Vec::with_capacity(num),
            genome: Genome::new(num),
        };

        for _ in result.into_iter() {
            result.pos.push(rand_range_vec2(x_min, x_max, y_min, y_max));
        }

        result
    }

    pub fn set_random_radius(&mut self, min: i32, max: i32) {
        self.radius = Vec::with_capacity(self.num);
        for _ in self.into_iter() {
            self.radius.push(rand_ranged_i32(min, max))
        }
    }

    pub fn actualize_rigidbodies(&mut self, rigidbody_set: &mut RigidBodySet) {
        while self.rigidbody.len() < self.num {
            let pos = self.pos[self.rigidbody.len()];
            let rb = RigidBodyBuilder::dynamic()
                .gravity_scale(0.0)
                .ccd_enabled(true)
                .position(Isometry::new(vector![pos.x, pos.y], 0.0))
                .build();
            self.rigidbody.push(rigidbody_set.insert(rb));
        }
    }

    pub fn actualize_colliders(&mut self, colliders_set: &mut ColliderSet, rigidbody_set: &mut RigidBodySet) {
        for i in self.into_iter() {
            if self.collider.len() > i {
                colliders_set.get_mut(self.collider[i]).unwrap().shape_mut().as_ball_mut().unwrap().radius = self.radius[i] as f32;
            }
            else {
                let radius = self.radius[i] as f32;
                let collider = ColliderBuilder::ball(radius)
                    .mass(4.0/3.0 * PI * radius * radius)
                    .build();
                let rb = self.rigidbody[i];
                self.collider.push(colliders_set.insert_with_parent(collider, rb, rigidbody_set));
            }
        }
    }

    pub fn draw<T: Fn(F32x2, i32) -> ()>(&self, draw_func: T) {
        for i in self.into_iter() {
            draw_func(self.pos[i], self.radius[i]);
        }
    }

    pub fn clamp_pos(&mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        for i in self.into_iter() {
            let mut cur = self.pos[i];
            cur.x = cur.x.clamp(x_min, x_max);
            cur.y = cur.y.clamp(y_min, y_max);
            self.pos[i] = cur;
        }
    }

    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.num
    }
}

impl Genome {
    pub fn new(length: usize) -> Genome {
        Genome {
            length,
            photosynth: vec![Gen::default(); length],
            movement_force: vec![Gen::default(); length],
            movement_rate: vec![Gen::default(); length],
        }
    }

    pub const fn empty() -> Genome {
        Genome {
            length: 0,
            photosynth: vec![],
            movement_force: vec![],
            movement_rate: vec![],
        }
    }

    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.length
    }

    pub fn fill_default(&mut self) {
        self.photosynth.fill_default();
        self.movement_force.fill_default();
        self.movement_rate.fill_default();
    }

    pub fn normilize(&mut self) {
        for i in self.into_iter() {
            let arr = [
                &mut self.photosynth[i],
                &mut self.movement_force[i],
                &mut self.movement_rate[i],
            ];

            let sum = arr.iter().map(|v| **v).sum::<f32>();
            for number in arr {
                *number /= sum;
            }
        }
    }
}

pub fn rand_range_vec2(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> F32x2 {
    F32x2{
        x: rand::thread_rng().gen_range(x_min..x_max),
        y: rand::thread_rng().gen_range(y_min..y_max),
    }
}

pub fn rand_ranged_i32(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_ranged_f32(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}