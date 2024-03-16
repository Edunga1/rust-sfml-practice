use sfml::{graphics::{Color, RectangleShape, Shape, Transformable}, system::Vector2f};

pub trait Moveable {
   fn move_shape(&mut self, vector: Vector2f);
}

pub struct Nemo<'a> {
    pub rect: RectangleShape<'a>,
    boundary: Option<(f32, f32)>,
}

impl Moveable for Nemo<'_> {
    fn move_shape(&mut self, vector: Vector2f) {
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

impl Nemo<'_> {
    pub fn new() -> Nemo<'static> {
        let mut rect = RectangleShape::new();
        rect.set_size((100., 100.));
        rect.set_position((200., 200.));
        rect.set_fill_color(Color::RED);
        rect.set_outline_color(Color::GREEN);
        rect.set_outline_thickness(3.);
        Nemo {
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
}

