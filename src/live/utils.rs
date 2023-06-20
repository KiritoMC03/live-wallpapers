use std::ops::Range;

use micromath::vector::F32x2;
use rand::Rng;

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