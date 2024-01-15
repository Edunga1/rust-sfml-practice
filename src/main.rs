use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Shape, RectangleShape, Transformable},
    window::{Event, Key, Style},
};

fn move_shape(shape: &mut RectangleShape, event: Event) {
    match event {
        Event::KeyPressed { code: Key::Left, .. } => { shape.set_position((shape.position().x - 10., shape.position().y)); }
        Event::KeyPressed { code: Key::Right, .. } => { shape.set_position((shape.position().x + 10., shape.position().y)); }
        Event::KeyPressed { code: Key::Up, .. } => { shape.set_position((shape.position().x, shape.position().y - 10.)); }
        Event::KeyPressed { code: Key::Down, .. } => { shape.set_position((shape.position().x, shape.position().y + 10.)); }
        _ => {}
    }
}

struct Nemo<'a> {
    rect: RectangleShape<'a>,
}

impl Nemo<'_> {
    fn new() -> Nemo<'static> {
        let mut rect = RectangleShape::new();
        rect.set_size((100., 100.));
        rect.set_position((200., 200.));
        rect.set_fill_color(Color::RED);
        rect.set_outline_color(Color::GREEN);
        rect.set_outline_thickness(3.);
        Nemo { rect }
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

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => return,
                Event::KeyPressed { code: Key::Left, .. }
                | Event::KeyPressed { code: Key::Right, .. }
                | Event::KeyPressed { code: Key::Up, .. }
                | Event::KeyPressed { code: Key::Down, .. } => move_shape(&mut nemo.rect, event),
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&nemo.rect);
        window.display();
    }
}


