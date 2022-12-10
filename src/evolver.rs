//! Manages the evolution of [Creature](crate::creature::Creature)s using [Simulation]s

use std::time::Duration;

use crate::{
    creature::{CreatureBuilder, Position},
    simulation::{Simulation, FLOOR_TOP_Y, STEPS_FREQUENCY, STEPS_PER_SECOND, WORLD_X_SIZE},
};

const SIMULATIONS_PER_GENERATION: i32 = 100;
const STEPS_PER_GENERATION: i32 = STEPS_PER_SECOND * 15;
const STEPS_PER_EVOLUTION: i32 = STEPS_PER_SECOND * 5;

/// Manages the evolution of [Creature](crate::creature::Creature)s using generations of [Simulation]s
pub struct Evolver {
    generations: Vec<Vec<Simulation>>,
    time_left_over: Duration,
    state: EvolverState,
}

impl Evolver {
    pub fn new() -> Evolver {
        let mut evolver = Evolver {
            generations: Vec::new(),
            time_left_over: Duration::ZERO,
            state: EvolverState::SimulatingGeneration {
                steps_left: STEPS_PER_GENERATION,
            },
        };

        evolver.generate_next_generation();

        evolver
    }

    pub fn state(&self) -> EvolverState {
        self.state
    }

    fn generate_next_generation(&mut self) {
        if self.generations.is_empty() {
            // Create first generation
            let mut generation = Vec::new();

            for _ in 0..SIMULATIONS_PER_GENERATION {
                generation.push(Simulation::new(
                    CreatureBuilder::random()
                        .translate_bottom_center_to(Position::new(WORLD_X_SIZE / 2.0, FLOOR_TOP_Y))
                        .build(),
                ))
            }

            self.generations.push(generation);

            return;
        }

        // Otherwise, improve last generation
        let generation = self.get_current_generation();

        panic!("Need to improve generation of size {}", generation.len())
    }

    pub fn get_current_generation(&self) -> &Vec<Simulation> {
        self.generations.last().unwrap()
    }

    fn step(&mut self) {
        match self.state {
            EvolverState::SimulatingGeneration { ref mut steps_left } => {
                *steps_left -= 1;
                if *steps_left <= 0 {
                    self.state = EvolverState::Evolving {
                        steps_left: STEPS_PER_EVOLUTION,
                    };

                    return;
                }
                for simulation in self.generations.last_mut().unwrap() {
                    simulation.step();
                }
            }
            EvolverState::Evolving { ref mut steps_left } => {
                *steps_left -= 1;
                if *steps_left <= 0 {
                    self.state = EvolverState::SimulatingGeneration {
                        steps_left: STEPS_PER_GENERATION,
                    };

                    self.generate_next_generation();
                }
            }
        };
    }

    pub fn run(&mut self, mut time: Duration) {
        time += self.time_left_over;

        while time > STEPS_FREQUENCY {
            time -= STEPS_FREQUENCY;
            self.step();
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

#[derive(Debug, Clone, Copy)]
pub enum EvolverState {
    SimulatingGeneration { steps_left: i32 },
    Evolving { steps_left: i32 },
}
