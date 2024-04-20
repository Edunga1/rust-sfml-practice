use std::{cell::RefCell, rc::Rc};

use game::game::{GameEvent, Game, MessageNotifier};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key},
};
use ui::{renderer::Renderer, window::create_window};

mod ui;

fn main() {
    let mut window = create_window();
    let mut game = Game::new();
    let renderer = Rc::new(RefCell::new(Renderer::new()));
    let time_per_frame = sfml::system::Time::seconds(1.0 / 60.0);
    let mut clock = sfml::system::Clock::start();
    let mut time_since_last_tick = sfml::system::Time::seconds(0.0);

    game.subscribe(renderer.clone());

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

        time_since_last_tick += clock.restart();
        while time_since_last_tick >= time_per_frame {
            time_since_last_tick -= time_per_frame;
            game.tick();
        }

        window.clear(Color::BLACK);

        for ele in game.units() {
            renderer.borrow_mut().draw_unit(&mut window, ele, &game.playable);
            renderer.borrow().draw_messages(&mut window);
        }

        window.display();
    }
}
