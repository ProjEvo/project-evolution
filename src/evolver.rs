//! Manages the evolution of [Creature](crate::creature::Creature)s using [Simulation]s

use std::time::Duration;

use crate::{
    creature::CreatureBuilder,
    simulation::{Simulation, MAX_WORLD_X, MAX_WORLD_Y, STEPS_FREQUENCY},
};

const SIMULATIONS_PER_GENERATION: i32 = 10;

/// Manages the evolution of [Creature](crate::creature::Creature)s using generations of [Simulation]s
pub struct Evolver {
    generations: Vec<Vec<Simulation>>,
    time_left_over: Duration,
}

impl Evolver {
    pub fn new() -> Evolver {
        let mut evolver = Evolver {
            generations: Vec::new(),
            time_left_over: Duration::ZERO,
        };

        evolver.generate_next_generation();

        evolver
    }

    fn generate_next_generation(&mut self) {
        if self.generations.is_empty() {
            // Create first generation
            let mut generation = Vec::new();

            for _ in 0..SIMULATIONS_PER_GENERATION {
                generation.push(Simulation::new(
                    CreatureBuilder::random()
                        .translate(MAX_WORLD_X / 2.0, MAX_WORLD_Y / 2.0)
                        .build(),
                ))
            }

            self.generations.push(generation);

            return;
        }

        // Otherwise, improve last generation
        let generation = self.get_current_generation();

        println!("Need to improve generation of size {}", generation.len())
    }

    pub fn get_current_generation(&self) -> &Vec<Simulation> {
        &self.generations.last().unwrap()
    }

    fn step_current_generation(&mut self) {
        for simulation in self.generations.last_mut().unwrap() {
            simulation.step();
        }
    }

    pub fn step(&mut self, mut time: Duration) {
        time += self.time_left_over;

        while time > STEPS_FREQUENCY {
            time -= STEPS_FREQUENCY;
            self.step_current_generation();
        }

        self.time_left_over = time;
    }
}

impl Default for Evolver {
    /// Functionally identical to [Evolver::new]
    fn default() -> Self {
        Self::new()
    }
}
