use micromath::vector::F32x2;
use rand::Rng;
use rand::SeedableRng;

#[derive(Default, Clone)]
pub struct Bacteries {
    pub num: usize,
    pub pos: Vec<F32x2>,
    pub velocity: Vec<F32x2>,
}

impl Bacteries {
    pub fn new(num: usize) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![F32x2::default(); num],
            velocity: vec![F32x2::default(); num],
        };

        result
    }

    pub const fn empty() -> Bacteries {
        let result = Bacteries {
            num: 0,
            pos: vec![],
            velocity: vec![],
        };

        result
    }

    pub fn rand_in_rect(num: usize, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Bacteries {
        let result = Bacteries {
            num,
            pos: vec![rand_vec2(x_min, x_max, y_min, y_max); num],
            velocity: vec![F32x2::default(); num],
        };

        result
    }

    pub fn draw<T: Fn(F32x2) -> ()>(&self, draw_func: T) {
        for i in self.into_iter() {
            draw_func(self.pos[i]);
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

    pub fn smart_move(&mut self, delta_time: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) {
        for i in self.into_iter() {
            let mut cur = self.pos[i];
            cur += self.velocity[i] * delta_time;
            cur.x = cur.x.clamp(x_min, x_max);
            cur.y = cur.y.clamp(y_min, y_max);
            self.pos[i] = cur;
        }
    }

    pub fn into_iter(&self) -> std::ops::Range<usize> {
        0..self.num
    }
}

pub fn rand_vec2(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> F32x2 {
    F32x2{
        x: rand::thread_rng().gen_range(x_min..x_max),
        y: rand::thread_rng().gen_range(y_min..y_max),
    }
}