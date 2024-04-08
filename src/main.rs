use game::game::{GameEvent, Game};
use sfml::{
    graphics::{Color, RenderTarget},
    window::{Event, Key},
};
use ui::{renderer::{get_text, Renderer}, window::create_window};

mod ui;

fn main() {
    let mut window = create_window();
    let mut game = Game::new();
    let renderer = Renderer::new();

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

        game.tick();
        window.clear(Color::BLACK);

        for ele in game.get_all_units() {
            renderer.draw_unit(&mut window, ele);
            // not working yet. why?
            let label = get_text(&ele.name, ele.pos.clone().into());
            window.draw(&label);
        }

        window.display();
    }
}
