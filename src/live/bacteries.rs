use std::{f32::consts::PI, ops::Range};

use micromath::vector::F32x2;
use rand::Rng;
use rapier2d::prelude::*;

use super::bacteries_processing::{START_ENERGY, DEAD_TIME, GENOME_MUT_RANGE, START_ALIVE_RANGE};

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

#[derive(Default, Clone)]
pub struct Genome {
    pub length: usize,
    pub photosynth: Vec<Gen>,
    pub carnivore: Vec<Gen>,
    pub movement_force: Vec<Gen>,
    pub movement_rate: Vec<Gen>,
}

pub type Gen = f32;

pub trait GenTrait {
    fn get(&self, idx: usize) -> Gen;
    fn fill_default(&mut self);
    fn default_one(&mut self, i: usize);
}

impl GenTrait for Vec<Gen> {
    #[inline(always)]
    fn get(&self, idx: usize) -> Gen {
        self[idx]
    }

    #[inline(always)]
    fn fill_default(&mut self) {
        for el in self {
            *el = rand_ranged_f32(0.0..1.0);
        }
    }

    #[inline(always)]
    fn default_one(&mut self, i: usize) {
         self[i] = rand_ranged_f32(0.0..1.0);
    }
}

impl Bacteries {
    #[inline(always)]
    pub fn new(num: usize) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![F32x2::default(); num],
            radius: vec![i32::default(); num],
            left_time: vec![START_ALIVE_RANGE.start; num],
            energy: vec![START_ENERGY; num],
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
    pub fn rand_in_rect(num: usize, capacity: usize, x: Range::<f32>, y: Range::<f32>) -> Bacteries {
        let mut result = Bacteries::new(capacity);

        for i in 0..num {
            result.left_time[i] = rand_ranged_f32(START_ALIVE_RANGE);
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
    pub fn actualize_rigidbodies(&mut self, rigidbody_set: &mut RigidBodySet) {
        while self.rigidbody.len() < self.num {
            let pos = self.pos[self.rigidbody.len()];
            let rb = RigidBodyBuilder::dynamic()
                .enabled(self.is_alive(self.rigidbody.len()))
                .gravity_scale(0.0)
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
                    .active_collision_types(ActiveCollisionTypes::all())
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
    pub fn is_dead(&self, idx: usize) -> bool {
        self.left_time[idx] <= DEAD_TIME
    }

    #[inline(always)]
    pub fn is_alive(&self, idx: usize) -> bool {
        self.left_time[idx] > DEAD_TIME
    }

    #[inline(always)]
    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.num
    }
}

impl Genome {
    pub fn new(length: usize) -> Genome {
        Genome {
            length,
            photosynth: vec![Gen::default(); length],
            carnivore: vec![Gen::default(); length],
            movement_force: vec![Gen::default(); length],
            movement_rate: vec![Gen::default(); length],
        }
    }

    pub const fn empty() -> Genome {
        Genome {
            length: 0,
            photosynth: vec![],
            carnivore: vec![],
            movement_force: vec![],
            movement_rate: vec![],
        }
    }

    pub fn mut_clone(&mut self, from: usize, to: usize) {
        self.photosynth[to] = self.photosynth[from] * rand_ranged_f32(GENOME_MUT_RANGE);
        self.carnivore[to] = self.carnivore[from] * rand_ranged_f32(GENOME_MUT_RANGE);
        self.movement_force[to] = self.movement_force[from] * rand_ranged_f32(GENOME_MUT_RANGE);
        self.movement_rate[to] = self.movement_rate[from] * rand_ranged_f32(GENOME_MUT_RANGE);
        self.normilize_one(to);
    }

    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.length
    }

    pub fn fill_default(&mut self) {
        self.photosynth.fill_default();
        self.carnivore.fill_default();
        self.movement_force.fill_default();
        self.movement_rate.fill_default();
        self.normilize();
    }

    pub fn normilize(&mut self) {
        for i in self.into_iter() {
            self.normilize_one(i)
        }
    }

    #[inline(always)]
    pub fn default_one(&mut self, i: usize) {
        self.photosynth.default_one(i);
        self.carnivore.default_one(i);
        self.movement_force.default_one(i);
        self.movement_rate.default_one(i);
        self.normilize_one(i);
    }

    #[inline(always)]
    pub fn normilize_one(&mut self, i: usize) {
        let arr = [
            &mut self.photosynth[i],
            &mut self.carnivore[i],
            &mut self.movement_force[i],
            &mut self.movement_rate[i],
        ];

        let sum = arr.iter().map(|v| **v).sum::<f32>();
        for number in arr {
            *number /= sum;
        }
    }
}


#[inline(always)]
pub fn rand_range_vec2(x: Range::<f32>, y: Range::<f32>) -> F32x2 {
    F32x2{
        x: rand::thread_rng().gen_range(x),
        y: rand::thread_rng().gen_range(y),
    }
}

#[inline(always)]
pub fn rand_ranged_i32(range: Range::<i32>) -> i32 {
    rand::thread_rng().gen_range(range)
}

#[inline(always)]
pub fn rand_ranged_f32(range: Range::<f32>) -> f32 {
    rand::thread_rng().gen_range(range)
}