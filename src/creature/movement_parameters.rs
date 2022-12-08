use std::{collections::HashMap, ops::RangeInclusive};

use rand::Rng;
use uuid::Uuid;

use crate::simulation::STEPS_PER_SECOND;

use super::{Muscle, Node};

type Range = RangeInclusive<i32>;

const OFFSET_RANGE: Range = 0..=STEPS_PER_SECOND * 1;

const EXTENSION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 4..=STEPS_PER_SECOND * 4;
const CONTRACTION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 4..=STEPS_PER_SECOND * 4;

/// Represents a set of parameters for when and how a muscle should move, in steps
pub struct MovementParameters {
    muscle_length: f32,
    offset: i32,
    extension_period: i32,
    contraction_period: i32,
}

impl MovementParameters {
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
                    offset: rng.gen_range(OFFSET_RANGE),

                    extension_period: rng.gen_range(EXTENSION_PERIOD_RANGE),
                    contraction_period: rng.gen_range(CONTRACTION_PERIOD_RANGE),
                },
            );
        }

        id_to_movement_parameters
    }

    pub fn muscle_length(&self) -> f32 {
        *&self.muscle_length
    }

    /// Gets the extension delta based on existing parameters
    /// 0.0 = Fully contracted
    /// 1.0 = Fully extended
    /// 0.5 = Normal
    pub fn get_extension_at(&self, step: i32) -> f32 {
        let total = self.extension_period + self.contraction_period;

        if step < self.offset {
            return 0.5;
        }

        let mut step_delta = (step - self.offset) % total;

        // Extension period
        if step_delta < self.extension_period {
            // First time, go from 0.5 to 1
            if step < self.offset + total {
                return (step_delta as f32 / self.extension_period as f32) * 0.5 + 0.5;
            }

            // All other times, go from 0 to 1
            return step_delta as f32 / self.extension_period as f32;
        }

        step_delta -= self.extension_period;

        // Contraction period (1 -> 0)
        return 1.0 - (step_delta as f32 / self.contraction_period as f32);
    }
}

impl Clone for MovementParameters {
    fn clone(&self) -> Self {
        Self {
            muscle_length: self.muscle_length,
            offset: self.offset,
            extension_period: self.extension_period,
            contraction_period: self.contraction_period,
        }
    }
}
