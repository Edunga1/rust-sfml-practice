use game::unit::unit::{Unit, Direction};
use sfml::graphics::{Color, RectangleShape, Shape, Transformable, Texture, RenderWindow, RenderTarget};

const SIZE: (f32, f32) = (50., 50.);

pub fn draw_unit(window: &mut RenderWindow, unit: &Unit) {
    let (mut x, y) = {
        let (x, y) = unit.pos.clone().into();
        (x as f32 * SIZE.0, y as f32 * SIZE.1)
    };
    let texture = Texture::from_file("src/resource/char-default.png").unwrap();
    let mut rect = RectangleShape::with_size((SIZE.0, SIZE.1).into());

    rect.set_texture(&texture, true);

    match unit.direction {
        Direction::Left => {
            rect.set_scale((-1., 1.));
            x += SIZE.0;
        }
        _ => {},
    }

    rect.set_position((x, y));
    window.draw(&rect);
}

pub fn get_text(text: &str, pos: (i32, i32)) -> sfml::graphics::Text {
    let (x, y) = {
        let (x, y) = pos;
        (x as f32 * 50., y as f32 * 50.)
    };
    let mut graphic = sfml::graphics::Text::default();
    graphic.set_string(text);
    graphic.set_character_size(24);
    graphic.set_fill_color(Color::WHITE);
    graphic.set_outline_color(Color::YELLOW);
    graphic.set_position((x, y));
    graphic
}
