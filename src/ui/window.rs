use sfml::{graphics::RenderWindow, system::Vector2i, window::Style};

pub fn create_window() -> RenderWindow {
    let mut obj = RenderWindow::new(
        (800, 600),
        "Game",
        Style::CLOSE,
        &Default::default(),
    );
    obj.set_vertical_sync_enabled(true);
    obj.set_position(Vector2i::new(0, 0));
    obj
}

