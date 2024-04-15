use std::ops;

use super::vector::Vector2d;

#[derive(PartialEq, Debug)]
pub struct Position {
    pub vector: Vector2d,
}

impl Clone for Position {
    fn clone(&self) -> Position {
        Position {
            vector: self.vector.clone(),
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            vector: Vector2d::new(x, y),
        }
    }

    pub fn move_(&mut self, vector: Vector2d) {
        self.vector = self.vector.clone().add(vector);
    }
}

impl From<Position> for (i32, i32) {
    fn from(pos: Position) -> (i32, i32) {
        (pos.vector.x, pos.vector.y)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            vector: self.vector.clone().add(rhs.vector.clone().mul(-1)),
        }
    }
}
