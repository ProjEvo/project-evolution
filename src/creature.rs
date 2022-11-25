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

/// A creature, made up of [Node]s and [Muscle]s. Contains a unique id for reference.
pub struct Creature {
    id: Uuid,
    nodes: HashMap<Uuid, Node>,
    muscles: HashMap<Uuid, Muscle>,
}

impl Creature {
    /// Creates a new empty creature
    pub fn new() -> Creature {
        Creature {
            id: Uuid::new_v4(),
            nodes: HashMap::new(),
            muscles: HashMap::new(),
        }
    }

    /// Creates a new random creature
    pub fn random() -> Creature {
        let mut rng = rand::thread_rng();

        let mut creature = Self::new();

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

            creature.add_node(Node::new(position, size))
        }

        let mut tested: HashMap<(Uuid, Uuid), bool> = HashMap::new();
        let mut muscles = Vec::new();

        for from in creature.nodes().values() {
            for to in creature.nodes().values() {
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

        creature.add_muscles(muscles);

        creature
    }

    /// Returns the unique id of the [Creature]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the nodes of the [Creature]
    pub fn nodes(&self) -> &HashMap<Uuid, Node> {
        &self.nodes
    }

    /// Returns the nodes of the [Creature], mutably
    pub fn nodes_mut(&mut self) -> &mut HashMap<Uuid, Node> {
        &mut self.nodes
    }

    /// Returns the unique id of the [Creature]
    pub fn muscles(&self) -> &HashMap<Uuid, Muscle> {
        &self.muscles
    }

    /// Returns the unique id of the [Creature]
    pub fn muscles_mut(&mut self) -> &mut HashMap<Uuid, Muscle> {
        &mut self.muscles
    }

    /// Adds a [Node] to the [Creature]
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    /// Adds a set of [Node]s to the [Creature]
    pub fn add_nodes(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.add_node(node);
        }
    }

    /// Adds a [Muscle] to the [Creature]
    pub fn add_muscle(&mut self, muscle: Muscle) {
        self.muscles.insert(muscle.id, muscle);
    }

    /// Adds a set of [Node]s to the [Creature]
    pub fn add_muscles(&mut self, muscles: Vec<Muscle>) {
        for muscle in muscles {
            self.add_muscle(muscle);
        }
    }

    /// Translates a creature x to the right and y down
    pub fn translate(&mut self, x: f32, y: f32) {
        for node in self.nodes_mut().values_mut() {
            node.position.x += x;
            node.position.y += y;
        }
    }

    /// Computes a creatures center and translates it to be centered around that position
    pub fn translate_center_to(&mut self, position: Position) {
        let mut total_nodes: f32 = 0.0;
        let mut x_sum: f32 = 0.0;
        let mut y_sum: f32 = 0.0;

        for node in self.nodes().values() {
            total_nodes += 1.0;

            x_sum += node.position.x;
            y_sum += node.position.y;
        }

        let translate_x = position.x - (x_sum / total_nodes);
        let translate_y = position.y - (y_sum / total_nodes);

        self.translate(translate_x, translate_y);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_creature() {
        let mut c = Creature::new();

        let nodes = Vec::from([
            Node::new(Position::new(1.0, 2.0), 3.0),
            Node::new(Position::new(2.0, 1.0), 3.0),
            Node::new(Position::new(5.0, 5.0), 3.0),
        ]);

        let id1 = nodes.get(0).unwrap().id;
        let id2 = nodes.get(1).unwrap().id;
        let id3 = nodes.get(2).unwrap().id;

        let muscles = Vec::from([Muscle::new(id1, id2), Muscle::new(id2, id3)]);
        let id4 = muscles.get(0).unwrap().id;

        c.add_nodes(nodes);
        c.add_muscles(muscles);

        assert_eq!(c.nodes().get(&id1).unwrap().position.x, 1.0);
        assert_eq!(c.nodes().get(&id3).unwrap().position.x, 5.0);
        assert_eq!(
            c.muscles().get(&id4).unwrap().to_id,
            c.nodes.get(&id2).unwrap().id
        );
    }
}
