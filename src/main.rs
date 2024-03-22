use game::game::GameEvent;
use sfml::{
    graphics::{Color, RenderTarget, RectangleShape, Shape, Transformable},
    window::{Event, Key},
};

mod ui;
mod game;

fn main() {
    let mut window = ui::window::create_window();
    let mut game = game::game::Game::new();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::Left, .. } => game.move_left(),
                Event::KeyPressed { code: Key::Right, .. } => game.move_right(),
                Event::KeyPressed { code: Key::Up, .. } => game.move_up(),
                Event::KeyPressed { code: Key::Down, .. } => game.move_down(),
                Event::KeyReleased { code: Key::Left, .. }
                | Event::KeyReleased { code: Key::Right, .. }
                | Event::KeyReleased { code: Key::Up, .. }
                | Event::KeyReleased { code: Key::Down, .. } => game.stop(),
                _ => {}
            }
        }

        let mut player_rect = RectangleShape::with_size(
            (game.playable.size as f32, game.playable.size as f32).into()
        );
        let pos: (i32, i32) = game.playable.pos.clone().into();
        player_rect.set_fill_color(Color::RED);
        player_rect.set_outline_color(Color::GREEN);
        player_rect.set_outline_thickness(3.);
        player_rect.set_position((pos.0 as f32, pos.1 as f32));

        game.tick();
        window.clear(Color::BLACK);
        window.draw(&player_rect);
        window.display();
    }
}
