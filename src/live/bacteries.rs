use micromath::vector::F32x2;
use rand::Rng;

pub struct Bactery {
    pos: F32x2,
    velocity: F32x2,
}

pub struct Bacteries(Vec<Bactery>);

impl Bactery {
    pub fn new(pos: F32x2) -> Bactery {
        Bactery {
            pos,
            velocity: F32x2::default(),
        }
    }
    
    pub fn with_velocity(pos: F32x2, velocity: F32x2) -> Bactery {
        Bactery {
            pos,
            velocity,
        }
    }
    
    pub fn move_next(&mut self, delta_time: f32) {
        self.pos += self.velocity * delta_time;
    }
}

impl Bacteries {
    pub fn rand_in_rect(num: u32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Bacteries {
        let mut result = Vec::with_capacity(num as usize);
        for _ in 0..num {
            result.push(Bactery::new(F32x2 {
                x: rand::thread_rng().gen_range(x_min..x_max),
                y: rand::thread_rng().gen_range(y_min..y_max),
            }));
        }
        
        Bacteries(result)
    }
    
    pub fn draw<T: Fn(F32x2) -> ()>(&self, draw_func: T) {
        for b in self.0.iter() {
            draw_func(b.pos);
        }
    }
}