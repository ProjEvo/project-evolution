use uuid::Uuid;

use super::Position;

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
