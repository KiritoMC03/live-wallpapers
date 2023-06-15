use rapier2d::na::Vector2;

use super::bacteries::{
    rand_ranged_i32,
    rand_range_vec2
};

use super::app::AppData;

pub const MOVE_RATE_SENS : i32 = 1000;
pub const MOVE_FORCE : f32 = 100.0;
pub const MIN_VEL : f32 = -1.0;
pub const MAX_VEL : f32 = 1.0;

pub fn process_genome(app_data: &mut AppData) {
    process_movement(app_data);
}

fn process_movement(app_data: &mut AppData) {
    let bac = &mut app_data.live_data.bacteries;
    let bodies = &mut app_data.physics_data.bodies;
    for i in bac.into_iter() {
        let rate = bac.genome.movement_rate[i] * MOVE_RATE_SENS as f32;
        
        if rate as i32 > rand_ranged_i32(0, MOVE_RATE_SENS) {
            let force = bac.genome.movement_force[i] * MOVE_FORCE;
            let vel = rand_range_vec2(MIN_VEL, MAX_VEL, MIN_VEL, MAX_VEL) * force;
            let vel_vec = Vector2::new(vel.x, vel.y);                
            bodies.get_mut(bac.rigidbody[i]).unwrap().add_force(vel_vec, true);
        }
    }
}