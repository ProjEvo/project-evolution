//! Contains the [Creature] struct and all related components of it

use std::{collections::HashMap, ops::RangeInclusive};

use rand::Rng;
use uuid::{self, Uuid};

const BASE_RANDOM_NODES: i32 = 3;
const RANDOM_CHANCE_TO_ADD_NODE: f32 = 0.25;
const RANDOM_NODE_X_POSITION_RANGE: RangeInclusive<f32> = -10.0..=10.0;
const RANDOM_NODE_Y_POSITION_RANGE: RangeInclusive<f32> = -10.0..=10.0;
const RANDOM_NODE_SIZE_RANGE: RangeInclusive<f32> = 1.0..=2.5;
const RANDOM_CHANGE_TO_CONNECT_NODES: f32 = 0.75;

/// A creature, made up of [Node]s and [Muscle]s. Contains a unique id for reference. Built using a [CreatureBuilder].
pub struct Creature {
    id: Uuid,
    nodes: HashMap<Uuid, Node>,
    muscles: HashMap<Uuid, Muscle>,
    muscle_lengths: HashMap<Uuid, f32>,
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

    /// Returns the muscles lengths of the [Creature], keyed by their id
    pub fn muscle_lengths(&self) -> &HashMap<Uuid, f32> {
        &self.muscle_lengths
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

        for from in (&creature_builder).nodes.values() {
            for to in (&creature_builder).nodes.values() {
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

    /// Computes a creatures center and translates it to be centered around that position
    pub fn translate_center_to(self, position: Position) -> CreatureBuilder {
        let mut total_nodes: f32 = 0.0;
        let mut x_sum: f32 = 0.0;
        let mut y_sum: f32 = 0.0;

        for node in self.nodes.values() {
            total_nodes += 1.0;

            x_sum += node.position.x;
            y_sum += node.position.y;
        }

        let translate_x = position.x - (x_sum / total_nodes);
        let translate_y = position.y - (y_sum / total_nodes);

        self.translate(translate_x, translate_y)
    }

    /// Builds the [CreatureBuilder] into a [Creature]
    pub fn build(self) -> Creature {
        let mut muscle_lengths = HashMap::new();

        for (id, muscle) in &self.muscles {
            let from = &self.nodes.get(&muscle.from_id).unwrap().position;
            let to = &self.nodes.get(&muscle.from_id).unwrap().position;
            muscle_lengths.insert(*id, from.distance_to(to));
        }

        Creature {
            id: self.id,
            nodes: self.nodes,
            muscles: self.muscles,
            muscle_lengths,
        }
    }
}

/// A node, defined by it's current position and size. Contains a unique id for reference.
pub struct Node {
    pub id: Uuid,
    pub position: Position,
    pub size: f32,
}

impl Node {
    /// Creates a new node at a [Position] with a certain size
    pub fn new<N: Into<f32>>(position: Position, size: N) -> Node {
        Node {
            id: Uuid::new_v4(),
            position,
            size: size.into(),
        }
    }
}

/// A muscle, defined by the ids of the two nodes it connects.  Contains a unique id for reference.
pub struct Muscle {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
}

impl Muscle {
    /// Creates a new muscle from one node to another using their ids
    pub fn new(from_id: Uuid, to_id: Uuid) -> Muscle {
        Muscle {
            id: Uuid::new_v4(),
            from_id,
            to_id,
        }
    }
}

/// A position in the 2D plane represented by an x and a y
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    /// Creates a new position at (x, y)
    pub fn new<N: Into<f32>>(x: N, y: N) -> Position {
        Position {
            x: x.into(),
            y: y.into(),
        }
    }

    /// Computes the distance from this position to `to`
    pub fn distance_to(&self, to: &Position) -> f32 {
        return f32::sqrt(f32::powi(self.x - to.x, 2) + f32::powi(self.y - to.y, 2));
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

    #[test]
    pub fn position_distance() {
        let pos1 = Position::new(5.0, 3.0);
        let pos2 = Position::new(0.0, 3.0);
        let pos3 = Position::new(5.0, 0.0);
        let pos4 = Position::new(3.0, 5.0);

        assert_eq!(pos1.distance_to(&pos2), 5.0);
        assert_eq!(pos1.distance_to(&pos3), 3.0);
        assert_eq!(pos1.distance_to(&pos4), f32::sqrt(8.0));
    }
}
