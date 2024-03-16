use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key},
};
use shape::nemo::Moveable;

mod ui;
mod shape;

fn main() {
    let mut window = ui::window::create_window();
    let mut nemo = shape::nemo::Nemo::new();
    nemo.set_boundary((800., 600.), true);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
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
