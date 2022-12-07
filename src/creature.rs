//! Contains the [Creature] struct and all related components of it

#[allow(clippy::module_inception)]
mod creature;
mod muscle;
mod node;
mod position;

pub use creature::{Creature, CreatureBuilder};
pub use muscle::Muscle;
pub use node::Node;
pub use position::Position;
