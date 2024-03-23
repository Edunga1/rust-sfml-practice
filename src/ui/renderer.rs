use sfml::graphics::{RectangleShape, Color, Shape, Transformable};

pub fn get_drawable(
    body: u32,
    pos: (i32, i32),
) -> RectangleShape<'static> {
    let _ = body;
    let mut rect = RectangleShape::with_size((50., 50.).into());
    rect.set_fill_color(Color::RED);
    rect.set_outline_color(Color::GREEN);
    rect.set_outline_thickness(3.);
    rect.set_position((pos.0 as f32, pos.1 as f32));
    rect
}
