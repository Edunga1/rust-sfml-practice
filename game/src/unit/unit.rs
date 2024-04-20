use crate::tick::counter::TickCounter;

use super::{position::Position, vector::Vector2d};

#[derive(PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub trait Moveable {
    fn move_(&mut self, direction: &Direction) -> bool;
}

pub struct Unit {
    pub name: String,
    pub pos: Position,
    pub body: u32,
    pub direction: Direction,
    boundary: Option<(i32, i32)>,
    movement_counter: TickCounter,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            name: String::from("noname"),
            pos: Position::new(0, 0),
            body: 1,
            direction: Direction::Right,
            boundary: None,
            movement_counter: (0, 20).into(),
        }
    }
}

impl Moveable for Unit {
    fn move_(&mut self, direction: &Direction) -> bool {
        if !self.movement_counter.reset() {
            return false;
        }

        if direction != &self.direction && matches!(direction, Direction::Left | Direction::Right) {
            self.direction = direction.clone();
            return true;
        }

        let vector = Unit::direction_to_vector(direction);
        if self.boundary.is_none() {
            self.pos.move_(vector);
            return true;
        }

        let (x, y) = {
            let (x, y) = self.pos.vector.clone().add(vector).into();
            let (mx, my) = self.boundary.unwrap();
            (x.clamp(0, mx), y.clamp(0, my))
        };
        self.pos = Position::new(x, y);
        true
    }
}

impl Unit {
    pub fn new() -> Unit {
        Unit {
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        self.movement_counter.tick();
    }

    pub fn set_boundary(&mut self, boundary: (i32, i32)) {
        self.boundary = Some(boundary);
    }

    fn direction_to_vector(direction: &Direction) -> Vector2d {
        match direction {
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
            Direction::Up => (0, -1).into(),
            Direction::Down => (0, 1).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_when_not_ready() {
        let mut unit = Unit::new();
        unit.pos = Position::new(0, 0);
        unit.direction = Direction::Left;

        unit.move_(&Direction::Left);

        assert_eq!(unit.pos, Position::new(0, 0));
    }

    #[test]
    fn test_move_when_ready() {
        let mut unit = Unit::new();
        unit.pos = Position::new(0, 0);
        unit.direction = Direction::Left;
        unit.movement_counter = (1, 1).into();

        unit.move_(&Direction::Left);

        assert_eq!(unit.pos, Position::new(-1, 0));
    }

    #[test]
    fn test_move_opposite_direction() {
        let mut unit = Unit::new();
        unit.pos = Position::new(0, 0);
        unit.direction = Direction::Left;
        unit.movement_counter = (1, 1).into();

        unit.move_(&Direction::Right);

        assert_eq!(unit.pos, Position::new(0, 0));
    }
}
