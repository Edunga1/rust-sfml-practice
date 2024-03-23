use sfml::graphics::{Color, RectangleShape, Shape, Transformable};

pub fn get_drawable(body: u32, pos: (i32, i32)) -> RectangleShape<'static> {
    let _ = body;
    let (x, y) = {
        let (x, y) = pos;
        (x as f32 * 5., y as f32 * 5.)
    };
    let mut rect = RectangleShape::with_size((50., 50.).into());
    rect.set_fill_color(Color::RED);
    rect.set_outline_color(Color::GREEN);
    rect.set_outline_thickness(3.);
    rect.set_position((x, y));
    rect
}
