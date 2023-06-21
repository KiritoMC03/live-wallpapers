use std::f32::consts::PI;

use micromath::vector::F32x2;
use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use crate::live::LiveData;

use super::app::AppData;
use super::utils::{rand_range_vec2, rand_ranged_f32};
use super::{normalize_f32x2, len_f32x2};

pub fn process_bacteries(app: &mut AppData) {
    process_alive(app);
    process_movement(app);

    if app.frame_num > 100 {
        process_photosynth(app);
        process_collisions(app);
        process_division(app);
        process_division_movement(app);
    }
}

fn process_alive(app: &mut AppData) {
    let live = &mut app.live_data;
    for i in live.bacteries.into_iter() {
        let mut left_time = live.bacteries.left_time[i];
        if left_time <= live.settings.dead_time {
            continue;
        }

        if calc_rate(live.bacteries.genome.live_regen_rate[i]){
            if left_time < live.settings.max_alive - live.settings.alive_to_energy_coef {
                let energy = &mut live.bacteries.energy[i];
                if *energy > 2.0 {
                    *energy -= 1.0;
                    left_time += live.settings.alive_to_energy_coef;
                }
            }
        }

        left_time -= app.delta_time;
        live.bacteries.left_time[i] = left_time;

        if left_time <= live.settings.dead_time {
            live.kill_bac(i);
        }
    }
}

fn process_movement(app: &mut AppData) {
    let vel_range = app.live_data.settings.vel_range.clone();
    let bac = &mut app.live_data.bacteries;
    for i in bac.into_iter() {
        if bac.is_dead(i, app.live_data.settings.dead_time) {
            continue;
        }

        if calc_rate(bac.genome.movement_rate[i]) {
            let force = bac.genome.movement_force[i] * app.live_data.settings.move_force;
            let vel = rand_range_vec2(vel_range.clone(), vel_range.clone()) * force;
            let vel_vec = Vector2::new(vel.x, vel.y);                
            app.live_data.physics_data.get_rb_mut(bac.rigidbody[i]).add_force(vel_vec, true);
        }
    }
}

fn process_photosynth(app: &mut AppData) {
    let live = &mut app.live_data;
    for i in live.bacteries.into_iter() {
        let photosynth = live.bacteries.genome.photosynth[i];
        if live.bacteries.is_dead(i, live.settings.dead_time) || photosynth == 0.0 {
            continue;
        }

        let radius = live.bacteries.radius[i];
        live.bacteries.energy[i] += photosynth * live.settings.photosynth_rate * app.delta_time * PI * (radius * radius) as f32;
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
        process_energy_distribution(app, a, b);
        process_repulsive(app, a, b);
    }
}

fn process_carnivore(app: &mut AppData, a: usize, b: usize) {
    let settings = &app.live_data.settings;
    let defence = settings.defence;
    let damage = settings.carnivore_damage;
    let rate = settings.carnivore_rate;
    let cost = settings.carnivore_cost;

    let bac = &mut app.live_data.bacteries;
    let cav_a = bac.genome.carnivore[a];
    let cav_b = bac.genome.carnivore[b];
    let dam_for_a = damage - bac.genome.defence[a] * defence;
    let dam_for_b = damage - bac.genome.defence[b] * defence;

    bac.left_time[a] -= (dam_for_a * (cav_b - cav_a).clamp(0.0, f32::MAX)) * app.delta_time;
    bac.left_time[b] -= (dam_for_b * (cav_a - cav_b).clamp(0.0, f32::MAX)) * app.delta_time;

    bac.energy[a] += (rate * rate - cost) * cav_a * app.delta_time;
    bac.energy[b] += (rate * rate - cost) * cav_b * app.delta_time;
}

fn process_energy_distribution(app: &mut AppData, a: usize, b: usize) {
    let en_distr = app.live_data.settings.max_energy_distribution;
    let bac = &mut app.live_data.bacteries;
    let dis_a = bac.genome.energy_distribution[a];
    let dis_b = bac.genome.energy_distribution[b];

    let a_to_b = dis_a * en_distr * app.delta_time;
    let b_to_a = dis_b * en_distr * app.delta_time;

    bac.energy[b] -= b_to_a;
    bac.energy[a] += b_to_a;

    bac.energy[a] -= a_to_b;
    bac.energy[b] += a_to_b;
}

fn process_repulsive(app: &mut AppData, a: usize, b: usize) {
    let data = &mut app.live_data;

    let pos_a = data.bacteries.pos[a];
    let pos_b = data.bacteries.pos[b];
    let mut a_to_b = pos_b - pos_a;
    normalize_f32x2(&mut a_to_b);

    try_repulsive(data, a, b, a_to_b);
    try_repulsive(data, b, a, a_to_b * -1.0);

    fn try_repulsive(data: &mut LiveData, cur: usize, other: usize, dir: F32x2) {
        if calc_rate(data.bacteries.genome.repulsive_rate[cur]) {
            let other_rb = data.bacteries.rigidbody[other];
            let force = data.bacteries.genome.repulsive_force[cur];
            let force = Vector2::new(dir.x, dir.y) * data.settings.max_repulsive_force * force;
            data.physics_data.get_rb_mut(other_rb).add_force(force, true);
        }
    }
}

fn process_division(app: &mut AppData) {
    let live = &mut app.live_data;
    for i in live.bacteries.into_iter() {
        if calc_rate(live.bacteries.genome.division_rate[i]) {
            let energy = &mut live.bacteries.energy[i];
            if *energy >= live.settings.division_energy {
                *energy -= live.settings.division_energy;
                live.mut_clone(i);
            }
        }
    }
}

fn process_division_movement(app: &mut AppData) {
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
        let pos = pos_a + offset + dir * 3.0 * app.delta_time;

        let rb = data.physics_data.get_rb_mut(data.bacteries.rigidbody[i]);
        rb.set_position(Isometry::new(vector![pos.x, pos.y], 0.0), true);
        if len_f32x2(&offset) > (rad_a + rad_b) as f32 {
            data.bacteries.is_parented[i] = false;
            rb.set_enabled(true);
        }
    }
}

fn calc_rate(rate: f32) -> bool {
    rate > rand_ranged_f32(0.0..1.0)
}