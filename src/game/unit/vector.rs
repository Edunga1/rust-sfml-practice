#[derive(Clone)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl Vector2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(self, other: Vector2d) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl From<Vector2d> for (i32, i32) {
    fn from(vector: Vector2d) -> (i32, i32) {
        (vector.x, vector.y)
    }
}

impl From<(i32, i32)> for Vector2d {
    fn from(tuple: (i32, i32)) -> Vector2d {
        Vector2d::new(tuple.0, tuple.1)
    }
}
