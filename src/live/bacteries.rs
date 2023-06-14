use micromath::vector::F32x2;
use rand::Rng;

use super::spatial_hash::SpatialHash;

pub struct Collision {
    pub a: usize,
    pub b: usize,
}

#[derive(Default)]
pub struct Bacteries {
    pub num: usize,
    pub pos: Vec<F32x2>,
    pub velocity: Vec<F32x2>,
    pub radius: Vec<i32>,
    pub spatial_hash: SpatialHash<usize>,
    pub genome: Genome,
}

#[derive(Default, Clone)]
pub struct Genome {
    pub length: usize,
    pub photosynth: Vec<Gen<f32>>
}

#[derive(Default, Clone, Copy)]
pub struct Gen<T> {
    pub value: T,
    pub weight: f32,
}

impl Bacteries {
    pub fn new(num: usize, cell_size: f32) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![F32x2::default(); num],
            velocity: vec![F32x2::default(); num],
            radius: vec![i32::default(); num],
            spatial_hash: SpatialHash::new(cell_size),
            genome: Genome::new(num),
        };

        result
    }

    pub const fn empty() -> Bacteries {
        let result = Bacteries {
            num: 0,
            pos: vec![],
            velocity: vec![],
            radius: vec![],
            spatial_hash: SpatialHash::empty(),
            genome: Genome::empty(),
        };

        result
    }

    pub fn rand_in_rect(num: usize, cell_size: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![rand_range_vec2(x_min, x_max, y_min, y_max); num],
            velocity: vec![F32x2::default(); num],
            radius: vec![0; num],
            spatial_hash: SpatialHash::new(cell_size),
            genome: Genome::new(num),
        };

        result
    }

    pub fn set_random_radius(&mut self, min: i32, max: i32) {
        self.radius = vec![rand_ranged_i32(min, max); self.num];
    }

    pub fn draw<T: Fn(F32x2, i32) -> ()>(&self, draw_func: T) {
        for i in self.into_iter() {
            draw_func(self.pos[i], self.radius[i]);
        }
    }

    pub fn move_all(&mut self, delta_time: f32)  {
        for i in self.into_iter() {
            self.pos[i] += self.velocity[i] * delta_time;
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

    pub fn detect_collisions(&self) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for i in self.into_iter() {
            match self.spatial_hash.querry_pos(self.pos[i]) {
                Some(others) => {
                    for j in others {
                        if i == *j {
                            continue;
                        }

                        if self.detect_collision(i, *j) {
                            collisions.push(Collision{
                                a: i,
                                b: *j,
                            });
                        }
                    }
                },
                None => {},
            }
        }

        collisions
    }

    fn detect_collision(&self, a: usize, b: usize) -> bool {
        let item_a = self.pos[a];
        let item_b = self.pos[b];
        let dx = item_a.x - item_b.x;
        let dy = item_a.y - item_b.y;

        let distance_squared = dx * dx + dy * dy;
        let sum_of_rad = self.radius[a] + self.radius[b];

        distance_squared <= (sum_of_rad * sum_of_rad) as f32
    }

    pub fn smart_move(&mut self, delta_time: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        for i in self.into_iter() {
            let mut cur = self.pos[i];
            cur += self.velocity[i] * delta_time;
            cur.x = cur.x.clamp(x_min, x_max);
            cur.y = cur.y.clamp(y_min, y_max);
            self.pos[i] = cur;
            self.spatial_hash.update_position(cur, i);
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
            photosynth: vec![Gen::new(0.0, 0.0); length],
        }
    }

    pub const fn empty() -> Genome {
        Genome {
            length: 0,
            photosynth: vec![],
        }
    }
}

impl<T> Gen<T> {
    pub const fn new(value: T, weight: f32) -> Gen<T> {
        Gen {
            value,
            weight,
        }
    }

    pub const fn empty(value: T) -> Gen<T> {
        Gen {
            value,
            weight: 0.0,
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