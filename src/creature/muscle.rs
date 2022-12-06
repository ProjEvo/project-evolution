use uuid::Uuid;

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
