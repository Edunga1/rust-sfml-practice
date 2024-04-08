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
            movement_counter: TickCounter::new(100),
        }
    }
}

impl Moveable for Unit {
    fn move_(&mut self, direction: &Direction) {
        if !self.movement_counter.reset() {
            return;
        }

        let vector = Unit::direction_to_vector(direction);
        if self.boundary.is_none() {
            self.pos.move_(vector);
            return;
        }

        let (x, y) = {
            let (x, y) = self.pos.vector.clone().add(vector).into();
            let (mx, my) = self.boundary.unwrap();
            (x.clamp(0, mx), y.clamp(0, my))
        };
        self.pos = Position::new(x, y);

        match direction {
            Direction::Left => self.direction = Direction::Left,
            Direction::Right => self.direction = Direction::Right,
            _ => {},
        }
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
