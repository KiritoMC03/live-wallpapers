use std::f32::consts::PI;
use std::ops::Range;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use super::bacteries::{
    rand_ranged_i32,
    rand_range_vec2,
};

use super::app::AppData;
use super::{normalize_f32x2, len_f32x2};

pub const MOVE_RATE_SENS : i32 = 1000;
pub const MOVE_FORCE : f32 = 100.0;
pub const VEL_RANGE : Range<f32> = -1.0..1.0;

pub const RADIUS_RANGE : Range<i32> = 4..14;

pub const MAX_ALIVE : f32 = 100.0;
pub const START_ALIVE_RANGE : Range<f32> = 1.0..100.0;
pub const DEAD_TIME : f32 = 0.0;

pub const START_ENERGY : f32 = 1.0;
pub const DIVISION_ENERGY : f32 = 10.0;
pub const ALIVE_TO_ENERGY_COEF : f32 = 0.1;

pub const PHOTOSYNTH : f32 = 0.01;
pub const CARNIVORE_RATE : f32 = 14.0;
pub const CARNIVORE_DAMAGE : f32 = 4.0;
pub const CARNIVORE_COST : f32 = 0.3;

pub const GENOME_MUT_RANGE : Range<f32> = 0.9..1.1;
pub const RADIUS_MUT_RANGE : Range<f32> = 0.9..1.1;

pub fn process_bacteries(app: &mut AppData) {
//    process_alive(app);
//    process_movement(app);
//
//    if app.frame_num > 100 {
//        process_photosynth(app);
//        process_collisions(app);
//        process_division(app);
//    }
}

fn process_alive(app: &mut AppData) {
    let live = &mut app.live_data;
    for i in live.bacteries.into_iter() {
        let mut left_time = live.bacteries.left_time[i];
        if left_time <= DEAD_TIME {
            continue;
        }

        if left_time < MAX_ALIVE - ALIVE_TO_ENERGY_COEF {
            let energy = &mut live.bacteries.energy[i];
            if *energy > 2.0 {
                *energy -= 1.0;
                left_time += ALIVE_TO_ENERGY_COEF;
            }
        }

        left_time -= app.delta_time;
        live.bacteries.left_time[i] = left_time;

        if left_time <= DEAD_TIME {
            live.kill_bac(i);
        }
    }
}

fn process_movement(app: &mut AppData) {
    let bac = &mut app.live_data.bacteries;
    for i in bac.into_iter() {
        if bac.is_dead(i) {
            continue;
        }

        let rate = bac.genome.movement_rate[i] * MOVE_RATE_SENS as f32;

        if rate as i32 > rand_ranged_i32(0..MOVE_RATE_SENS) {
            let force = bac.genome.movement_force[i] * MOVE_FORCE;
            let vel = rand_range_vec2(VEL_RANGE, VEL_RANGE) * force;
            let vel_vec = Vector2::new(vel.x, vel.y);                
            app.live_data.physics_data.get_rb_mut(bac.rigidbody[i]).add_force(vel_vec, true);
        }
    }
}

fn process_photosynth(app: &mut AppData) {
    let live = &mut app.live_data;
    for i in live.bacteries.into_iter() {
        let photosynth = live.bacteries.genome.photosynth[i];
        if live.bacteries.is_dead(i) || photosynth == 0.0 {
            continue;
        }

        let radius = live.bacteries.radius[i];
        live.bacteries.energy[i] += photosynth * PHOTOSYNTH * app.delta_time * PI * (radius * radius) as f32;
    }
}

fn process_collisions(app: &mut AppData) {
    if app.live_data.physics_data.events.collisions.is_none() {
        return;
    }

    let mut collisions = app.live_data.physics_data.events.collisions.take().unwrap();
    for col in collisions.iter_mut() {
        let physics = &mut app.live_data.physics_data;
        let a = physics.get_coll(col.collider1()).user_data as usize;
        let b = physics.get_coll(col.collider2()).user_data as usize;
        process_carnivore(app, a, b);
    }
}

fn process_carnivore(app: &mut AppData, a: usize, b: usize) {
    let bac = &mut app.live_data.bacteries;
    let cav_a = bac.genome.carnivore[a];
    let cav_b = bac.genome.carnivore[b];
    //    let asp = cav_a / cav_b;
    //    if asp > 2.0 && cav_a > 0.5 {
    //        println!("ARGG {a}, enerty: {}", bac.energy[a]);
    //    }
    //    else if 1.0 / asp > 2.0 && cav_b > 0.5 {
    //        println!("ARGG {b}, enerty: {}", bac.energy[b]);
    //    }
    bac.left_time[a] -= (CARNIVORE_DAMAGE * (cav_b - cav_a).clamp(0.0, f32::MAX)) * app.delta_time;
    bac.left_time[b] -= (CARNIVORE_DAMAGE * (cav_a - cav_b).clamp(0.0, f32::MAX)) * app.delta_time;

    bac.energy[a] += (CARNIVORE_RATE - CARNIVORE_COST) * cav_a * app.delta_time;
    bac.energy[b] += (CARNIVORE_RATE - CARNIVORE_COST) * cav_b * app.delta_time;
}

fn process_division(app: &mut AppData) {
    for i in app.live_data.bacteries.into_iter() {
        let live = &mut app.live_data;
        let energy = &mut live.bacteries.energy[i];
        if *energy >= DIVISION_ENERGY {
            *energy -= DIVISION_ENERGY;
            live.mut_clone(i);
        }
    }

    let data = &mut app.live_data;
    for i in data.bacteries.into_iter() {
        if !data.bacteries.is_parented[i] { continue; }

        let parent = data.bacteries.parent[i];
        let rad_a = data.bacteries.radius[parent];
        let rad_b = data.bacteries.radius[i];
        let pos_a = data.bacteries.pos[parent];
        let pos_b = data.bacteries.pos[i];
        let offset = pos_b - pos_a;
        let mut dir = offset;
        normalize_f32x2(&mut dir);
        let pos = pos_a + offset + dir * app.delta_time;

        let rb = data.physics_data.get_rb_mut(data.bacteries.rigidbody[i]);
        rb.set_position(Isometry::new(vector![pos.x, pos.y], 0.0), true);
        if len_f32x2(&offset) > (rad_a + rad_b) as f32 {
            data.bacteries.is_parented[i] = false;
            rb.set_enabled(true);
        }
    }
}