//! Contains the [Creature] struct and all related components of it

use uuid::{self, Uuid};

/// A creature, made up of [Node]s and [Muscle]s. Contains a unique id for reference.
pub struct Creature {
    pub id: Uuid,
    pub nodes: Vec<Node>,
    pub muscles: Vec<Muscle>,
}

impl Creature {
    /// Creates a new creature from a vector of [Node]s and a vector of [Muscle]s
    pub fn new(nodes: Vec<Node>, muscles: Vec<Muscle>) -> Creature {
        Creature {
            id: Uuid::new_v4(),
            nodes,
            muscles,
        }
    }
}

/// A node, defined by it's current position and size. Contains a unique id for reference.
pub struct Node {
    pub id: Uuid,
    pub position: Position,
    pub size: f64,
}

impl Node {
    /// Creates a new node at a [Position] with a certain size
    pub fn new<N: Into<f64>>(position: Position, size: N) -> Node {
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
    pub x: f64,
    pub y: f64,
}

impl Position {
    /// Creates a new position at (x, y)
    pub fn new<N: Into<f64>>(x: N, y: N) -> Position {
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
        let nodes = Vec::from([
            Node::new(Position::new(1, 2), 3),
            Node::new(Position::new(2, 1), 3),
            Node::new(Position::new(5, 5), 3),
        ]);

        let id1 = nodes.get(0).unwrap().id;
        let id2 = nodes.get(1).unwrap().id;
        let id3 = nodes.get(2).unwrap().id;

        let muscles = Vec::from([Muscle::new(id1, id2), Muscle::new(id2, id3)]);

        let c = Creature::new(nodes, muscles);

        assert_eq!(c.nodes.get(0).unwrap().position.x, 1.0);
        assert_eq!(c.nodes.get(2).unwrap().position.x, 5.0);
        assert_eq!(c.muscles.get(1).unwrap().to_id, c.nodes.get(2).unwrap().id);
    }
}
