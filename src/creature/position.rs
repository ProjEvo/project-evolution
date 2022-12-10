/// A position in the 2D plane represented by an x and a y
#[derive(Debug, Clone, Copy)]
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

    /// Computes the distance from this position to `to` position
    ///
    /// # Example
    /// ```
    /// use project_evolution::creature::Position;
    ///
    /// let pos1 = Position::new(3.0, 0.0);
    /// let pos2 = Position::new(0.0, 4.0);
    /// assert_eq!(pos1.distance_to(&pos2), 5.0)
    /// ```
    pub fn distance_to(&self, to: &Position) -> f32 {
        f32::sqrt(f32::powi(self.x - to.x, 2) + f32::powi(self.y - to.y, 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn distance_to() {
        let pos1 = Position::new(5.0, 3.0);
        let pos2 = Position::new(0.0, 3.0);
        let pos3 = Position::new(5.0, 0.0);
        let pos4 = Position::new(3.0, 5.0);

        assert_eq!(pos1.distance_to(&pos2), 5.0);
        assert_eq!(pos1.distance_to(&pos3), 3.0);
        assert_eq!(pos1.distance_to(&pos4), f32::sqrt(8.0));
    }
}
