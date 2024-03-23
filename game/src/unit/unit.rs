use crate::tick::counter::TickCounter;

use super::{position::Position, vector::Vector2d};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub trait Moveable {
    fn move_(&mut self, direction: &Direction);
}

pub struct Unit {
    pub pos: Position,
    pub size: i32,
    boundary: Option<(i32, i32)>,
    movement_counter: TickCounter,
}

impl Moveable for Unit {
    fn move_(&mut self, direction: &Direction) {
        if !self.movement_counter.reset() {
            return;
        }

        let vector = Unit::direction_to_vector(direction).mul(5);
        if self.boundary.is_none() {
            self.pos.move_(vector);
            return;
        }

        let (x, y) = self.pos.vector.clone().add(vector).into();
        let (x, y) = (x.max(0), y.max(0));
        let (x, y) = (
            x.min(self.boundary.unwrap().0),
            y.min(self.boundary.unwrap().1),
        );
        self.pos = Position::new(x, y);
    }
}

impl Unit {
    pub fn new() -> Unit {
        Unit {
            pos: Position::new(200, 200),
            size: 100,
            boundary: None,
            movement_counter: TickCounter::new(30),
        }
    }

    pub fn tick(&mut self) {
        self.movement_counter.tick();
    }

    pub fn set_boundary(&mut self, boundary: (i32, i32), aware_size: bool) {
        self.boundary = Some(if aware_size {
            let (x, y) = (boundary.0 - self.size, boundary.1 - self.size);
            (x, y)
        } else {
            boundary
        });
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
