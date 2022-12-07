use std::{collections::HashMap, ops::RangeInclusive};

use rand::Rng;
use uuid::Uuid;

use crate::simulator::STEPS_PER_SECOND;

use super::{Muscle, Node};

type Range = RangeInclusive<i32>;

const OFFSET_RANGE: Range = 0..=STEPS_PER_SECOND * 12;

const EXTENSION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 2..=STEPS_PER_SECOND * 4;
const EXTENSION_SLEEP_PERIOD_RANGE: Range = 0..=STEPS_PER_SECOND * 8;
const EXTENSION_RETURN_PERIOD_RANGE: Range = STEPS_PER_SECOND / 2..=STEPS_PER_SECOND * 4;
const EXTENSION_RETURN_SLEEP_PERIOD_RANGE: Range = 0..=STEPS_PER_SECOND * 8;

const CONTRACTION_PERIOD_RANGE: Range = STEPS_PER_SECOND / 2..=STEPS_PER_SECOND * 4;
const CONTRACTION_SLEEP_PERIOD_RANGE: Range = 0..=STEPS_PER_SECOND * 8;
const CONTRACTION_RETURN_PERIOD_RANGE: Range = STEPS_PER_SECOND / 2..=STEPS_PER_SECOND * 4;
const CONTRACTION_RETURN_SLEEP_PERIOD_RANGE: Range = 0..=STEPS_PER_SECOND * 8;

/// Represents a set of parameters for when and how a muscle should move, in steps
pub struct MovementParameters {
    muscle_length: f32,

    offset: i32,

    extension_period: i32,
    extension_sleep_period: i32,
    extension_return_period: i32,
    extension_return_sleep_period: i32,

    contraction_period: i32,
    contraction_sleep_period: i32,
    contraction_return_period: i32,
    contraction_return_sleep_period: i32,
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
                    extension_sleep_period: rng.gen_range(EXTENSION_SLEEP_PERIOD_RANGE),
                    extension_return_period: rng.gen_range(EXTENSION_RETURN_PERIOD_RANGE),
                    extension_return_sleep_period: rng
                        .gen_range(EXTENSION_RETURN_SLEEP_PERIOD_RANGE),

                    contraction_period: rng.gen_range(CONTRACTION_PERIOD_RANGE),
                    contraction_sleep_period: rng.gen_range(CONTRACTION_SLEEP_PERIOD_RANGE),
                    contraction_return_period: rng.gen_range(CONTRACTION_RETURN_PERIOD_RANGE),
                    contraction_return_sleep_period: rng
                        .gen_range(CONTRACTION_RETURN_SLEEP_PERIOD_RANGE),
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
        let total = self.extension_period
            + self.extension_sleep_period
            + self.extension_return_period
            + self.extension_return_sleep_period
            + self.contraction_period
            + self.contraction_sleep_period
            + self.contraction_return_period
            + self.contraction_return_sleep_period;

        if step < self.offset {
            return 0.5;
        }

        let mut step_delta = (step - self.offset) % total;

        if step_delta < self.extension_period {
            return 0.5 + (((step_delta as f32) / (self.extension_period as f32)) * 0.5);
        }

        step_delta -= self.extension_period;

        if step_delta < self.extension_sleep_period {
            return 1.0;
        }

        step_delta -= self.extension_sleep_period;

        if step_delta < self.extension_return_period {
            return 1.0 - (((step_delta as f32) / (self.extension_return_period as f32)) * 0.5);
        }

        step_delta -= self.extension_return_period;

        if step_delta < self.extension_return_sleep_period {
            return 0.5;
        }

        step_delta -= self.extension_return_sleep_period;

        if step_delta < self.contraction_period {
            return 0.5 - (((step_delta as f32) / (self.contraction_period as f32)) * 0.5);
        }

        step_delta -= self.contraction_period;

        if step_delta < self.contraction_sleep_period {
            return 0.0;
        }

        step_delta -= self.contraction_sleep_period;

        if step_delta < self.contraction_return_period {
            return ((step_delta as f32) / (self.contraction_return_period as f32)) * 0.5;
        }

        return 0.5;
    }
}

impl Clone for MovementParameters {
    fn clone(&self) -> Self {
        Self {
            muscle_length: self.muscle_length,
            offset: self.offset.clone(),
            extension_period: self.extension_period.clone(),
            extension_sleep_period: self.extension_sleep_period.clone(),
            extension_return_period: self.extension_return_period.clone(),
            extension_return_sleep_period: self.extension_return_sleep_period.clone(),
            contraction_period: self.contraction_period.clone(),
            contraction_sleep_period: self.contraction_sleep_period.clone(),
            contraction_return_period: self.contraction_return_period.clone(),
            contraction_return_sleep_period: self.contraction_return_sleep_period.clone(),
        }
    }
}
