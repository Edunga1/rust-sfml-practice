use game::unit::unit::{Direction, Unit};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Texture, Transformable},
    SfBox,
};
use std::collections::HashMap;

const SIZE: (f32, f32) = (50., 50.);

pub struct Renderer {
    body_texture: HashMap<u32, SfBox<Texture>>,
}

impl Renderer {
    const DEFAULT_BODY: u32 = 0;

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Self::DEFAULT_BODY, Texture::from_file("src/resource/char-default.png").unwrap());
        Self { body_texture: map }
    }

    pub fn draw_unit(&self, window: &mut RenderWindow, unit: &Unit) {
        self.draw_body(window, unit);
    }

    fn get_texture(&self, id: u32) -> &Texture {
        if self.body_texture.contains_key(&id) {
            return self.body_texture.get(&id).unwrap();
        }
        self.body_texture.get(&Self::DEFAULT_BODY).unwrap()
    }

    fn draw_body(&self, window: &mut RenderWindow, unit: &Unit) {
        let (mut x, y) = {
            let (x, y) = unit.pos.clone().into();
            (x as f32 * SIZE.0, y as f32 * SIZE.1)
        };
        let texture = self.get_texture(unit.body);
        let mut rect = RectangleShape::with_size((SIZE.0, SIZE.1).into());

        rect.set_texture(&texture, true);

        match unit.direction {
            Direction::Left => {
                rect.set_scale((-1., 1.));
                x += SIZE.0;
            }
            _ => {}
        }

        rect.set_position((x, y));
        window.draw(&rect);
    }
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
