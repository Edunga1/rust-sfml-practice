use sfml::graphics::{Color, RectangleShape, Shape, Transformable};

pub fn get_drawable(body: u32, pos: (i32, i32)) -> RectangleShape<'static> {
    let _ = body;
    let (x, y) = {
        let (x, y) = pos;
        (x as f32 * 50., y as f32 * 50.)
    };
    let mut rect = RectangleShape::with_size((50., 50.).into());
    rect.set_fill_color(Color::RED);
    rect.set_outline_color(Color::GREEN);
    rect.set_outline_thickness(3.);
    rect.set_position((x, y));
    rect
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
