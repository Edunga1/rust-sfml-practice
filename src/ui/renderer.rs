use game::unit::unit::{Direction, Unit};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Texture, Transformable},
    SfBox,
};
use std::collections::HashMap;

const SIZE: (f32, f32) = (50., 50.);

pub struct Renderer {
    body_texture: HashMap<u32, SfBox<Texture>>,
    font: SfBox<sfml::graphics::Font>,
}

impl Renderer {
    const DEFAULT_BODY: u32 = 0;

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            Self::DEFAULT_BODY,
            Texture::from_file("src/resource/char-default.png").unwrap(),
        );
        let font = sfml::graphics::Font::from_file("src/resource/font.ttf").unwrap();
        Self { body_texture: map, font }
    }

    pub fn draw_unit(&self, window: &mut RenderWindow, unit: &Unit) {
        self.draw_body(window, unit);
        self.draw_name(window, unit);
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

    fn draw_name(&self, window: &mut RenderWindow, unit: &Unit) {
        let (x, y) = {
            let (x, y) = unit.pos.clone().into();
            (x as f32 * SIZE.0, y as f32 * SIZE.1)
        };
        let mut text = sfml::graphics::Text::default();
        text.set_font(&self.font);
        text.set_string(&unit.name);
        text.set_character_size(10);
        text.set_fill_color(Color::WHITE);
        text.set_position((x, y));
        // set alignment to center
        let bounds = text.global_bounds();
        text.set_origin((bounds.width / 2., bounds.height));
        window.draw(&text);
    }
}
