use std::ops::Mul;

use sfml::{graphics::{Color, RectangleShape, Shape, Transformable}, system::Vector2f};

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub trait Moveable {
   fn move_shape(&mut self, direction: &Direction);
}

pub struct Unit<'a> {
    pub rect: RectangleShape<'a>,
    boundary: Option<(f32, f32)>,
}

impl Moveable for Unit<'_> {
    fn move_shape(&mut self, direction: &Direction) {
        let vector = Unit::direction_to_vector(direction).mul(5.);
        if self.boundary.is_none() {
            self.rect.move_(vector);
            return;
        }

        let (x, y) = (self.rect.position().x + vector.x, self.rect.position().y + vector.y);
        let (x, y) = (x.max(0.), y.max(0.));
        let (x, y) = (x.min(self.boundary.unwrap().0), y.min(self.boundary.unwrap().1));
        self.rect.set_position((x, y));
    }
}

impl Unit<'_> {
    pub fn new() -> Unit<'static> {
        let mut rect = RectangleShape::new();
        rect.set_size((100., 100.));
        rect.set_position((200., 200.));
        rect.set_fill_color(Color::RED);
        rect.set_outline_color(Color::GREEN);
        rect.set_outline_thickness(3.);
        Unit {
            rect,
            boundary: None,
        }
    }

    pub fn set_boundary(&mut self, boundary: (f32, f32), aware_size: bool) {
        self.boundary = Some(
            if aware_size {
                let size = self.rect.size();
                let (x, y) = (boundary.0 - size.x, boundary.1 - size.y);
                (x, y)
            } else {
                boundary
            }
        );
    }

    fn direction_to_vector(direction: &Direction) -> Vector2f {
        match direction {
            Direction::Left => (-1., 0.).into(),
            Direction::Right => (1., 0.).into(),
            Direction::Up => (0., -1.).into(),
            Direction::Down => (0., 1.).into(),
        }
    }
}

