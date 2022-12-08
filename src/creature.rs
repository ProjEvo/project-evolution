//! Contains the [Creature] struct and all related components of it

#[allow(clippy::module_inception)]
mod creature;
mod creature_colors;
mod movement_parameters;
mod muscle;
mod node;
mod position;

pub use creature::{Creature, CreatureBuilder};
pub use creature_colors::CreatureColors;
pub use movement_parameters::MovementParameters;
pub use muscle::Muscle;
pub use node::Node;
pub use position::Position;
