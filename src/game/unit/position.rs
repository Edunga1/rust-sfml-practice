use super::vector::Vector2d;

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
