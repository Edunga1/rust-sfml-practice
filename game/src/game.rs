use super::unit::unit::{Direction, Moveable, Unit};
use rand::Rng;

pub trait GameEvent {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn stop(&mut self);
}

pub struct Game {
    pub serial: Option<u32>, // unused yet
    pub playable: Unit,
    units: Vec<Unit>,
    playable_direction: Option<Direction>,
}

impl Default for Game {
    fn default() -> Self {
        let player = {
            let mut unit = Unit::new();
            unit.set_boundary((200, 100));
            unit
        };
        Self {
            serial: Option::None,
            playable: player,
            units: vec![],
            playable_direction: Option::None,
        }
    }
}

impl GameEvent for Game {
    fn move_left(&mut self) {
        self.playable_direction = Option::Some(Direction::Left);
    }

    fn move_right(&mut self) {
        self.playable_direction = Option::Some(Direction::Right);
    }

    fn move_up(&mut self) {
        self.playable_direction = Option::Some(Direction::Up);
    }

    fn move_down(&mut self) {
        self.playable_direction = Option::Some(Direction::Down);
    }

    fn stop(&mut self) {
        self.playable_direction = Option::None;
    }
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            ..Default::default()
        };
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut unit = Unit::new();
            unit.pos.move_((rng.gen_range(0..15), rng.gen_range(0..12)).into());
            game.add_unit(unit);
        }
        game
    }

    pub fn tick(&mut self) {
        self.move_playable();
        self.playable.tick();
    }

    pub fn get_all_units(&self) -> Vec<&Unit> {
        let mut units = self.units.iter().collect::<Vec<&Unit>>();
        units.push(&self.playable);
        units
    }

    fn move_playable(&mut self) {
        if let Some(ref direction) = self.playable_direction {
            self.playable.move_(direction);
        } else {
            return;
        }
    }

    fn add_unit(&mut self, unit: Unit) {
        self.units.push(unit);
    }
}
