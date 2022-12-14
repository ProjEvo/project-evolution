use std::{collections::HashMap, ops::RangeInclusive};

use rand::Rng;
use uuid::Uuid;

use crate::{simulation::STEPS_PER_SECOND, util};

use super::{Muscle, Node};

type Range = RangeInclusive<i32>;

const EXTENSION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 4..=STEPS_PER_SECOND * 4;
const CONTRACTION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 4..=STEPS_PER_SECOND * 4;

const MUTATE_EXTENSION_PERIOD_RANGE: Range = -STEPS_PER_SECOND / 30..=STEPS_PER_SECOND / 30;
const MUTATE_CONTRACTION_PERIOD_RANGE: Range = -STEPS_PER_SECOND / 30..=STEPS_PER_SECOND / 30;

/// Represents a set of parameters for when and how a muscle should move, in steps
#[derive(Debug)]
pub struct MovementParameters {
    muscle_length: f32,
    extension_period: i32,
    contraction_period: i32,
}

impl MovementParameters {
    /// Generates for a set of muscles and nodes
    pub fn generate_for_muscles_and_nodes(
        muscles: &HashMap<Uuid, Muscle>,
        nodes: &HashMap<Uuid, Node>,
    ) -> HashMap<Uuid, MovementParameters> {
        let mut rng = rand::thread_rng();
        let mut id_to_movement_parameters = HashMap::new();

        for (id, muscle) in muscles {
            let from = &nodes.get(&muscle.from_id).unwrap().position;
            let to = &nodes.get(&muscle.to_id).unwrap().position;
            let muscle_length = from.distance_to(to);

            id_to_movement_parameters.insert(
                *id,
                MovementParameters {
                    muscle_length,

                    extension_period: rng.gen_range(EXTENSION_PERIOD_RANGE),
                    contraction_period: rng.gen_range(CONTRACTION_PERIOD_RANGE),
                },
            );
        }

        id_to_movement_parameters
    }

    /// Creates a new MovementParameters that is a mutation of the passed in one
    pub fn mutate(movement_parameters: &MovementParameters) -> MovementParameters {
        let mut rng = rand::thread_rng();

        let new_extension_period = util::clamp_to_range(
            movement_parameters.extension_period + rng.gen_range(MUTATE_EXTENSION_PERIOD_RANGE),
            EXTENSION_PERIOD_RANGE,
        );

        let new_contraction_period = util::clamp_to_range(
            movement_parameters.contraction_period + rng.gen_range(MUTATE_CONTRACTION_PERIOD_RANGE),
            CONTRACTION_PERIOD_RANGE,
        );

        MovementParameters {
            muscle_length: movement_parameters.muscle_length,

            extension_period: new_extension_period,
            contraction_period: new_contraction_period,
        }
    }

    /// Gets the normal muscle length
    pub fn muscle_length(&self) -> f32 {
        self.muscle_length
    }

    /// Returns true if extending, false if contracting
    pub fn is_extending(&self, step: i32) -> bool {
        let total = self.extension_period + self.contraction_period;

        let step_delta = step % total;

        step_delta < self.extension_period
    }

    /// Gets the extension delta based on existing parameters
    /// 0.0 = Fully contracted
    /// 1.0 = Fully extended
    /// 0.5 = Normal
    pub fn get_extension_at(&self, step: i32) -> f32 {
        let total = self.extension_period + self.contraction_period;

        let mut step_delta = step % total;

        // Extension period
        if step_delta < self.extension_period {
            // First time, go from 0.5 to 1
            if step < total {
                return (step_delta as f32 / self.extension_period as f32) * 0.5 + 0.5;
            }

            // All other times, go from 0 to 1
            return step_delta as f32 / self.extension_period as f32;
        }

        step_delta -= self.extension_period;

        // Contraction period (1 -> 0)
        1.0 - (step_delta as f32 / self.contraction_period as f32)
    }
}
