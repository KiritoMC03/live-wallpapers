use std::iter;

use super::{utils::rand_ranged_f32, bacteries_processing::GENOME_MUT_RANGE};

pub type Gen = f32;

#[derive(Default, Clone)]
pub struct Genome {
    pub length: usize,
    pub live_regen_rate: Vec<Gen>,
    pub division_rate: Vec<Gen>,
    pub photosynth: Vec<Gen>,
    pub carnivore: Vec<Gen>,
    pub movement_force: Vec<Gen>,
    pub movement_rate: Vec<Gen>,
    pub defence: Vec<Gen>,
    pub energy_distribution: Vec<Gen>,
    pub repulsive_force: Vec<Gen>,
    pub repulsive_rate: Vec<Gen>,
}

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

impl Genome {
    /// Return new normalized Genome with default gens (random in 0..1)
    #[inline(always)]
    pub fn new(length: usize) -> Genome {
        let mut result = Genome {
            length,
            live_regen_rate: default_gen(length),
            division_rate: default_gen(length),
            photosynth: default_gen(length),
            carnivore: default_gen(length),
            movement_force: default_gen(length),
            movement_rate: default_gen(length),
            defence: default_gen(length),
            energy_distribution: default_gen(length),
            repulsive_force: default_gen(length),
            repulsive_rate: default_gen(length),
        };

        result.normilize();
        result
    }

    #[inline(always)]
    pub const fn empty() -> Genome {
        Genome {
            length: 0,
            live_regen_rate: vec![],
            division_rate: vec![],
            photosynth: vec![],
            carnivore: vec![],
            movement_force: vec![],
            movement_rate: vec![],
            defence: vec![],
            energy_distribution: vec![],
            repulsive_force: vec![],
            repulsive_rate: vec![],
        }
    }

    #[inline(always)]
    pub fn mut_clone(&mut self, from: usize, to: usize) {
        for gen in self.iter_mut() {
            gen[to] = gen[from] * rand_ranged_f32(GENOME_MUT_RANGE);
        }
        self.normilize_one(to);
    }

    #[inline(always)]
    pub fn default_one(&mut self, i: usize) {
        for gen in self.iter_mut() {
            gen.default_one(i);
        }
        self.normilize_one(i);
    }

    #[inline(always)]
    pub fn normilize_one(&mut self, i: usize) {
        let sum = self.iter().map(|v| v[i]).sum::<f32>();
        for number in self.iter_mut().map(|v| &mut v[i]) {
            *number /= sum;
        }
    }

    #[inline(always)]
    pub fn normilize(&mut self) {
        for i in self.iter_idxs() {
            self.normilize_one(i);
        }
    }

    pub fn iter_idxs(&self) -> std::ops::Range<usize> {
        0..self.length
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<f32>> {
        assert!(Self::is_iter_correct(create(self).count()), "Check Genome.iter(), is not correct!");
        return create(self);

        fn create(genome: &Genome) -> impl Iterator<Item = &Vec<f32>> {
            iter::once(&genome.live_regen_rate)
                .chain(iter::once(&genome.division_rate))
                .chain(iter::once(&genome.photosynth))
                .chain(iter::once(&genome.carnivore))
                .chain(iter::once(&genome.movement_force))
                .chain(iter::once(&genome.movement_rate))
                .chain(iter::once(&genome.defence))
                .chain(iter::once(&genome.energy_distribution))
                .chain(iter::once(&genome.repulsive_force))
                .chain(iter::once(&genome.repulsive_rate))
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<f32>> {
        assert!(Self::is_iter_correct(create(self).count()), "Check Genome.iter(), is not correct!");
        return create(self);
        
        fn create(genome: &mut Genome) -> impl Iterator<Item = &mut Vec<f32>> {
            iter::once(&mut genome.live_regen_rate)
                .chain(iter::once(&mut genome.division_rate))
                .chain(iter::once(&mut genome.photosynth))
                .chain(iter::once(&mut genome.carnivore))
                .chain(iter::once(&mut genome.movement_force))
                .chain(iter::once(&mut genome.movement_rate))
                .chain(iter::once(&mut genome.defence))
                .chain(iter::once(&mut genome.energy_distribution))
                .chain(iter::once(&mut genome.repulsive_force))
                .chain(iter::once(&mut genome.repulsive_rate))            
        }
    }

    fn is_iter_correct(iter_len: usize) -> bool {
        std::mem::size_of::<Genome>() ==
        std::mem::size_of::<Vec<f32>>() * iter_len + std::mem::size_of::<usize>()
    }
}

pub fn default_gen(length: usize) -> Vec<Gen> {
    let mut res = vec![Gen::default(); length];
    res.fill_default();
    res
}