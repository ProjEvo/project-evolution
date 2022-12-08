use std::{collections::HashMap, ops::RangeInclusive};

use egui::Color32;
use rand::Rng;
use uuid::{self, Uuid};

use crate::util;

use super::{CreatureColors, MovementParameters, Muscle, Node, Position};

const BASE_RANDOM_NODES: i32 = 3;
const RANDOM_CHANCE_TO_ADD_NODE: f32 = 0.25;
const RANDOM_NODE_X_POSITION_RANGE: RangeInclusive<f32> = -100.0..=100.0;
const RANDOM_NODE_Y_POSITION_RANGE: RangeInclusive<f32> = -100.0..=100.0;
const RANDOM_NODE_SIZE_RANGE: RangeInclusive<f32> = 10.0..=20.0;
const RANDOM_CHANGE_TO_CONNECT_NODES: f32 = 0.75;
const COLOR_H_RANGE: RangeInclusive<u16> = 0..=300;

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
}

impl CreatureBuilder {
    /// Creates a [CreatureBuilder]
    pub fn new() -> CreatureBuilder {
        CreatureBuilder {
            id: Uuid::new_v4(),
            nodes: HashMap::new(),
            muscles: HashMap::new(),
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
    pub fn translate_bottom_center_to(self, position: Position) -> CreatureBuilder {
        let mut total_nodes: f32 = 0.0;
        let mut x_sum: f32 = 0.0;
        let mut y_max: f32 = f32::MIN;

        for node in self.nodes.values() {
            total_nodes += 1.0;

            x_sum += node.position.x;

            let this_y_max = node.position.y + (node.size / 2.0);

            if this_y_max > y_max {
                y_max = this_y_max
            }
        }

        let translate_x = position.x - (x_sum / total_nodes);
        let translate_y = position.y - y_max;

        self.translate(translate_x, translate_y)
    }

    /// Builds the [CreatureBuilder] into a [Creature]
    pub fn build(self) -> Creature {
        let mut rng = rand::thread_rng();

        let movement_parameters =
            MovementParameters::generate_for_muscles_and_nodes(&self.muscles, &self.nodes);

        let h = rng.gen_range(COLOR_H_RANGE);
        let (nr, ng, nb) = util::hsv_to_rgb(h, 75, 100);
        let (er, eg, eb) = util::hsv_to_rgb(h, 75, 75);
        let (cr, cg, cb) = util::hsv_to_rgb(h, 75, 50);

        let colors = CreatureColors {
            node_color: Color32::from_rgb(nr, ng, nb),
            muscle_extended: Color32::from_rgb(er, eg, eb),
            muscle_contracted: Color32::from_rgb(cr, cg, cb),
        };

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
