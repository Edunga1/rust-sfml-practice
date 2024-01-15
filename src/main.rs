use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Shape, RectangleShape, Transformable},
    window::{Event, Key, Style}, system::Vector2f,
};

trait Moveable {
    fn move_shape(&mut self, vector: Vector2f);
}

struct Nemo<'a> {
    rect: RectangleShape<'a>,
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
    fn new() -> Nemo<'static> {
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

    fn set_boundary(&mut self, boundary: (f32, f32), aware_size: bool) {
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

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut nemo = Nemo::new();
    nemo.set_boundary((800., 600.), true);

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => return,
                Event::KeyPressed { code: Key::Left, .. } => nemo.move_shape((-10., 0.).into()),
                Event::KeyPressed { code: Key::Right, .. } => nemo.move_shape((10., 0.).into()),
                Event::KeyPressed { code: Key::Up, .. } => nemo.move_shape((0., -10.).into()),
                Event::KeyPressed { code: Key::Down, .. } => nemo.move_shape((0., 10.).into()),
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&nemo.rect);
        window.display();
    }
}


