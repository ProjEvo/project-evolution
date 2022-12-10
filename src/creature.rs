//! Contains the [Creature] struct and all related components of it

#[allow(clippy::module_inception)]
mod creature_colors;
mod movement_parameters;
mod muscle;
mod node;
mod position;

pub use creature_colors::CreatureColors;
pub use movement_parameters::MovementParameters;
pub use muscle::Muscle;
pub use node::Node;
pub use position::Position;

use std::{collections::HashMap, ops::RangeInclusive};

use rand::Rng;
use uuid::{self, Uuid};

use crate::util;

const BASE_RANDOM_NODES: i32 = 3;
const RANDOM_CHANCE_TO_ADD_NODE: f32 = 0.25;
const RANDOM_NODE_X_POSITION_RANGE: RangeInclusive<f32> = -100.0..=100.0;
const RANDOM_NODE_Y_POSITION_RANGE: RangeInclusive<f32> = -100.0..=100.0;
const RANDOM_NODE_SIZE_RANGE: RangeInclusive<f32> = 10.0..=20.0;
const RANDOM_CHANGE_TO_CONNECT_NODES: f32 = 0.75;

/// A creature, made up of [Node]s and [Muscle]s. Contains a unique id for reference. Built using a [CreatureBuilder].
pub struct Creature {
    id: Uuid,
    nodes: HashMap<Uuid, Node>,
    muscles: HashMap<Uuid, Muscle>,
    movement_parameters: HashMap<Uuid, MovementParameters>,
    colors: CreatureColors,
}

impl Creature {
    /// Returns the unique id of the [Creature]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the nodes of the [Creature]
    pub fn nodes(&self) -> &HashMap<Uuid, Node> {
        &self.nodes
    }

    /// Returns the unique id of the [Creature]
    pub fn muscles(&self) -> &HashMap<Uuid, Muscle> {
        &self.muscles
    }

    /// Returns the movement parameters of the [Creature]'s [Muscle]s, keyed by their id
    pub fn movement_parameters(&self) -> &HashMap<Uuid, MovementParameters> {
        &self.movement_parameters
    }

    /// Returns the node color
    pub fn colors(&self) -> &CreatureColors {
        &self.colors
    }
}

/// Builds a [Creature]
pub struct CreatureBuilder {
    id: Uuid,
    nodes: HashMap<Uuid, Node>,
    muscles: HashMap<Uuid, Muscle>,
    movement_parameters: Option<HashMap<Uuid, MovementParameters>>,
    colors: Option<CreatureColors>,
}

impl CreatureBuilder {
    /// Creates a [CreatureBuilder]
    pub fn new() -> CreatureBuilder {
        CreatureBuilder {
            id: Uuid::new_v4(),
            nodes: HashMap::new(),
            muscles: HashMap::new(),
            movement_parameters: None,
            colors: None,
        }
    }

    /// Creates a [CreatureBuilder], and adds random nodes and muscles
    pub fn random() -> CreatureBuilder {
        let mut rng = rand::thread_rng();

        let mut creature_builder = Self::new();

        let mut number_of_nodes = BASE_RANDOM_NODES;

        while rng.gen::<f32>() < RANDOM_CHANCE_TO_ADD_NODE {
            number_of_nodes += 1;
        }

        for _ in 0..number_of_nodes {
            let position = Position::new(
                rng.gen_range(RANDOM_NODE_X_POSITION_RANGE),
                rng.gen_range(RANDOM_NODE_Y_POSITION_RANGE),
            );

            let size = rng.gen_range(RANDOM_NODE_SIZE_RANGE);

            creature_builder = creature_builder.add_node(Node::new(position, size));
        }

        let mut tested: HashMap<(Uuid, Uuid), bool> = HashMap::new();

        let mut muscles = Vec::new();

        for from in creature_builder.nodes.values() {
            for to in creature_builder.nodes.values() {
                if from.id == to.id || tested.contains_key(&(to.id, from.id)) {
                    continue;
                }

                if rng.gen::<f32>() >= RANDOM_CHANGE_TO_CONNECT_NODES {
                    continue;
                }

                tested.insert((from.id, to.id), true);

                muscles.push(Muscle::new(from.id, to.id));
            }
        }

        for muscle in muscles {
            creature_builder = creature_builder.add_muscle(muscle)
        }

        creature_builder
    }

    /// Creates a [CreatureBuilder] by building off a previous [Creature] and mutating it.
    ///
    /// This method binds new Uuids to all objects out of necessity.
    pub fn mutate(creature: &Creature) -> CreatureBuilder {
        let mut builder = CreatureBuilder::new();

        // Need to map old uuids to the new ones
        let mut old_uuid_to_new_uuid: HashMap<Uuid, Uuid> = HashMap::new();

        // Duplicate nodes
        for (old_id, node) in creature.nodes() {
            let new_node = Node::new(node.position, node.size);

            old_uuid_to_new_uuid.insert(*old_id, new_node.id);

            builder = builder.add_node(new_node);
        }

        // Duplicate muscles and movement parameters
        let mut movement_parameters = HashMap::new();

        for (old_id, muscle) in creature.muscles() {
            let new_muscle = Muscle::new(
                old_uuid_to_new_uuid[&muscle.from_id],
                old_uuid_to_new_uuid[&muscle.to_id],
            );

            movement_parameters.insert(
                new_muscle.id,
                MovementParameters::mutate(&creature.movement_parameters()[old_id]),
            );

            builder = builder.add_muscle(new_muscle);
        }

        // Add MovementParameters and CharacterColors, then return
        builder
            .add_movement_parameters(movement_parameters)
            .add_colors(CreatureColors::mutate(&creature.colors))
    }

    /// Adds a [Node] to the [Creature]
    pub fn add_node(mut self, node: Node) -> CreatureBuilder {
        self.nodes.insert(node.id, node);

        self
    }

    /// Adds a [Muscle] to the [Creature]
    pub fn add_muscle(mut self, muscle: Muscle) -> CreatureBuilder {
        self.muscles.insert(muscle.id, muscle);

        self
    }

    /// Sets the movement parameters of the [Creature]'s [Muscle]s, as keyed by their ids
    pub fn add_movement_parameters(
        mut self,
        movement_parameters: HashMap<Uuid, MovementParameters>,
    ) -> CreatureBuilder {
        self.movement_parameters = Some(movement_parameters);

        self
    }

    /// Sets the [colors](CreatureColors) of the [Creature]
    pub fn add_colors(mut self, colors: CreatureColors) -> CreatureBuilder {
        self.colors = Some(colors);

        self
    }

    /// Gets the bounds of the [Creature], represented by the top left and bottom right
    fn get_bounds(&self) -> (Position, Position) {
        let x_pos_iter = self.nodes.values().map(|node| node.position.x);
        let y_pos_iter = self.nodes.values().map(|node| node.position.y);

        (
            Position::new(
                x_pos_iter.clone().min_by(util::cmp_f32).unwrap(),
                y_pos_iter.clone().min_by(util::cmp_f32).unwrap(),
            ),
            Position::new(
                x_pos_iter.max_by(util::cmp_f32).unwrap(),
                y_pos_iter.max_by(util::cmp_f32).unwrap(),
            ),
        )
    }

    /// Translates a creature x to the right and y down
    pub fn translate(mut self, x: f32, y: f32) -> CreatureBuilder {
        for node in &mut self.nodes.values_mut() {
            node.position.x += x;
            node.position.y += y;
        }

        self
    }

    /// Computes a creatures bottom center and translates it's bottom center to that position
    ///
    /// # Explanation
    /// For example, if a creature looked like:
    ///
    /// ```text
    /// A --- B
    ///  \
    ///   \
    ///    C
    /// ```
    ///
    /// Translating to a point P would put C there
    pub fn translate_bottom_center_to(self, position: &Position) -> CreatureBuilder {
        let (top_left, bottom_right) = self.get_bounds();

        let translate_x = position.x - ((top_left.x + bottom_right.x) / 2.0);
        let translate_y = position.y - bottom_right.y;

        self.translate(translate_x, translate_y)
    }

    /// Builds the [CreatureBuilder] into a [Creature]
    pub fn build(self) -> Creature {
        let movement_parameters = self.movement_parameters.unwrap_or_else(|| {
            MovementParameters::generate_for_muscles_and_nodes(&self.muscles, &self.nodes)
        });

        let colors = self.colors.unwrap_or_else(|| CreatureColors::new());

        Creature {
            id: self.id,
            nodes: self.nodes,
            muscles: self.muscles,
            movement_parameters,
            colors,
        }
    }
}

impl Default for CreatureBuilder {
    /// Same as [CreatureBuilder::new]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_creature() {
        let node1 = Node::new(Position::new(1.0, 2.0), 3.0);
        let node2 = Node::new(Position::new(2.0, 1.0), 3.0);
        let node3 = Node::new(Position::new(5.0, 5.0), 3.0);

        let id1 = *&node1.id;
        let id2 = *&node2.id;
        let id3 = *&node3.id;

        let muscle1 = Muscle::new(id1, id2);
        let muscle2 = Muscle::new(id2, id3);

        let id4 = *&muscle1.id;

        let c = CreatureBuilder::new()
            .add_node(node1)
            .add_node(node2)
            .add_node(node3)
            .add_muscle(muscle1)
            .add_muscle(muscle2)
            .build();

        assert_eq!(c.nodes().get(&id1).unwrap().position.x, 1.0);
        assert_eq!(c.nodes().get(&id3).unwrap().position.x, 5.0);
        assert_eq!(
            c.muscles().get(&id4).unwrap().to_id,
            c.nodes.get(&id2).unwrap().id
        );
    }
}
