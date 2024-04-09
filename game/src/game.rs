use crate::unit::creature::{Creature, Squirrel};

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
    pub creatures: Vec<Box<dyn Creature>>,
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
            creatures: vec![],
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
            let mut squirrel = Squirrel::new();
            squirrel.unit.pos.move_((rng.gen_range(0..15), rng.gen_range(0..12)).into());
            game.creatures.push(Box::new(squirrel));
        }
        game
    }

    pub fn tick(&mut self) {
        self.move_playable();
        self.playable.tick();
    }

    pub fn units(&self) -> UnitsIter {
        UnitsIter::new(self)
    }

    fn move_playable(&mut self) {
        if let Some(ref direction) = self.playable_direction {
            self.playable.move_(direction);
        } else {
            return;
        }
    }
}

pub struct UnitsIter<'a> {
    game: &'a Game,
    index: usize,
    player_vsited: bool,
}

impl UnitsIter<'_> {
    pub fn new(game: &Game) -> UnitsIter {
        UnitsIter {
            game,
            index: 0,
            player_vsited: false,
        }
    }
}

impl<'a> Iterator for UnitsIter<'a> {
    type Item = &'a Unit;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.player_vsited {
            self.player_vsited = true;
            return Option::Some(&self.game.playable);
        }

        if self.index < self.game.creatures.len() {
            let unit = self.game.creatures.get(self.index).unwrap().get_unit();
            self.index += 1;
            return Option::Some(unit);
        }

        Option::None
    }
}
