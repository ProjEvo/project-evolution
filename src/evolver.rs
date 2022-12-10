//! Manages the evolution of [Creature](crate::creature::Creature)s using [Simulation]s

use std::time::Duration;

use crate::{
    creature::{CreatureBuilder, Position},
    simulation::{Simulation, FLOOR_TOP_Y, STEPS_FREQUENCY, STEPS_PER_SECOND, WORLD_X_SIZE},
};

const SIMULATIONS_PER_GENERATION: i32 = 100;
const STEPS_PER_GENERATION: i32 = STEPS_PER_SECOND * 15;
const STEPS_PER_EVOLUTION: i32 = STEPS_PER_SECOND * 5;
// Note that the top (SIMULATIONS_PER_GENERATION / OFFSPRING_PER_CREATURE) simulations will be picked for mutation. MUST BE > 1.
const OFFSPRING_PER_CREATURE: i32 = 2;

/// Manages the evolution of [Creature](crate::creature::Creature)s using generations of [Simulation]s
pub struct Evolver {
    current_generation: Vec<Simulation>,
    on_generation: usize,
    generation_scores: Vec<Vec<f32>>,
    time_left_over: Duration,
    state: EvolverState,
}

impl Evolver {
    /// Creates a new Evolver
    pub fn new() -> Evolver {
        let mut evolver = Evolver {
            current_generation: Vec::new(),
            on_generation: 0,
            generation_scores: Vec::new(),
            time_left_over: Duration::ZERO,
            state: EvolverState::SimulatingGeneration {
                steps_left: STEPS_PER_GENERATION,
            },
        };

        evolver.generate_next_generation();

        evolver
    }

    /// Gets the current state of the Evolver
    pub fn state(&self) -> EvolverState {
        self.state
    }

    /// Generates the next generation from the current one or randomly if the first generation
    fn generate_next_generation(&mut self) {
        let bottom_center = Position::new(WORLD_X_SIZE / 2.0, FLOOR_TOP_Y);
        if self.on_generation == 0 {
            // Create first generation
            let mut generation = Vec::new();

            for _ in 0..SIMULATIONS_PER_GENERATION {
                generation.push(Simulation::new(
                    CreatureBuilder::random()
                        .translate_bottom_center_to(&bottom_center)
                        .build(),
                ))
            }

            self.current_generation = generation;
            self.on_generation += 1;

            return;
        }

        // Otherwise, improve last generation
        let sorted_generation = &mut self.current_generation;
        sorted_generation.sort_by(|a, b| b.get_score().total_cmp(&a.get_score()));

        let old_scores = sorted_generation.iter().map(|s| s.get_score()).collect();
        self.generation_scores.push(old_scores);

        let mut new_generation = Vec::new();

        for simulation in sorted_generation.iter() {
            if new_generation.len() as i32 >= SIMULATIONS_PER_GENERATION {
                break;
            }

            let old_creature = simulation.creature();

            for _ in 0..OFFSPRING_PER_CREATURE {
                let builder = CreatureBuilder::mutate(old_creature);

                new_generation.push(Simulation::new(
                    builder.translate_bottom_center_to(&bottom_center).build(),
                ));
            }
        }

        self.current_generation = new_generation;
        self.on_generation += 1;
    }

    /// Gets the current generation
    pub fn current_generation(&self) -> &Vec<Simulation> {
        &self.current_generation
    }

    /// Returns the number of the current generation
    pub fn on_generation(&self) -> usize {
        self.on_generation
    }

    /// Gets the stored scores for past generations
    pub fn generation_scores(&self) -> &Vec<Vec<f32>> {
        &self.generation_scores
    }

    /// Steps the evolver
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
                for simulation in &mut self.current_generation {
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

    /// Runs the evolver for a certain amount of time
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

/// Represents the current state of an evolver
#[derive(Debug, Clone, Copy)]
pub enum EvolverState {
    SimulatingGeneration { steps_left: i32 },
    Evolving { steps_left: i32 },
}
