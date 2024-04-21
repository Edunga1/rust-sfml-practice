use game::{
    game::MessageReceiver,
    unit::unit::{Direction, Unit},
};
use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Texture, Transformable},
    system::Vector2u,
    SfBox,
};
use std::collections::HashMap;

const SIZE: (f32, f32) = (50., 50.);
const MAX_JOURNAL_MESSAGES: usize = 5;

pub struct Renderer {
    journal: Vec<String>,
    body_texture: HashMap<u32, SfBox<Texture>>,
    font: SfBox<sfml::graphics::Font>,
}

impl MessageReceiver for Renderer {
    fn notify(&mut self, message: &str) {
        self.journal.push(message.to_string());
    }
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
        let journal = vec![];
        Self {
            body_texture: map,
            font,
            journal,
        }
    }

    pub fn draw_unit(&self, window: &mut RenderWindow, unit: &Unit, protagonist: &Unit) {
        self.draw_body(window, unit, protagonist);
        self.draw_name(window, unit, protagonist);
        self.draw_grid(window);
    }

    pub fn draw_journal(&self, window: &mut RenderWindow) {
        let mut text = sfml::graphics::Text::default();
        text.set_font(&self.font);
        text.set_character_size(10);
        text.set_fill_color(Color::WHITE);
        text.set_position((0., 0.));
        let mut journal = self.journal.iter().rev().take(MAX_JOURNAL_MESSAGES);
        let mut y = 0.;
        while let Some(message) = journal.next() {
            text.set_string(message);
            text.set_position((0., y));
            window.draw(&text);
            y += 10.;
        }
    }

    fn get_texture(&self, id: u32) -> &Texture {
        if self.body_texture.contains_key(&id) {
            return self.body_texture.get(&id).unwrap();
        }
        self.body_texture.get(&Self::DEFAULT_BODY).unwrap()
    }

    fn draw_body(&self, window: &mut RenderWindow, unit: &Unit, protagonist: &Unit) {
        let (mut x, y) = self.calc_position(window, unit, protagonist);
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

    fn draw_name(&self, window: &mut RenderWindow, unit: &Unit, protagonist: &Unit) {
        let (x, y) = self.calc_position(window, unit, protagonist);
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

    fn calc_position(
        &self,
        window: &mut RenderWindow,
        unit: &Unit,
        protagonist: &Unit,
    ) -> (f32, f32) {
        let (sx, sy) = {
            let (x, y) = Vector(window.size()).into();
            (x / 2., y / 2.)
        };
        let (x, y) = (unit.pos.clone() - protagonist.pos.clone()).into();
        let x = (x as f32 * SIZE.0) + sx - SIZE.0 / 2.;
        let y = (y as f32 * SIZE.1) + sy - SIZE.1 / 2.;
        (x, y)
    }

    fn draw_grid(&self, window: &mut RenderWindow) {
        let (w, h) = Vector(window.size()).into();
        let (sx, sy) = (w / 2., h / 2.);
        let mut line = RectangleShape::with_size((1., h).into());
        line.set_fill_color(Color::rgba(255, 255, 255, 2));
        line.set_position((sx, 0.));
        window.draw(&line);
        line.set_size((w, 1.));
        line.set_position((0., sy));
        window.draw(&line);
    }
}

struct Vector(Vector2u);
impl From<Vector> for (f32, f32) {
    fn from(v: Vector) -> Self {
        (v.0.x as f32, v.0.y as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_into_tuple() {
        let v = Vector(Vector2u { x: 1, y: 2 });
        let t: (f32, f32) = v.into();
        assert_eq!(t, (1., 2.));
    }
}
