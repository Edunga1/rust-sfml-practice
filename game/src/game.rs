use std::{cell::RefCell, rc::Rc};

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

pub trait MessageReceiver {
    fn notify(&mut self, message: &str);
}

pub trait MessageNotifier<'a> {
    fn subscribe(&mut self, receiver: Rc<RefCell<dyn MessageReceiver + 'a>>);
    fn send(&mut self, message: &str);
}

pub struct Game<'a> {
    pub serial: Option<u32>, // unused yet
    pub playable: Unit,
    pub creatures: Vec<Box<dyn Creature>>,
    playable_direction: Option<Direction>,
    receivers: Vec<Rc<RefCell<dyn MessageReceiver + 'a>>>,
}

impl Default for Game<'_> {
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
            receivers: vec![],
        }
    }
}

impl GameEvent for Game<'_> {
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

impl<'a> MessageNotifier<'a> for Game<'a> {
    fn subscribe(&mut self, receiver: Rc<RefCell<dyn MessageReceiver + 'a>>) {
        self.receivers.push(receiver);
    }

    fn send(&mut self, message: &str) {
        for receiver in &self.receivers{
            receiver.borrow_mut().notify(message);
        }
    }
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
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

    pub fn units(&self) -> Vec<&Unit> {
        let mut units = vec![&self.playable];
        for creature in &self.creatures {
            units.push(&creature.get_unit());
        }
        units
    }

    fn move_playable(&mut self) {
        if let Some(ref direction) = self.playable_direction {
            if self.playable.move_(direction) {
                self.send("player moved");
            }
        } else {
            return;
        }
    }

    pub fn attack(&mut self) {
        if self.playable.attack() {
            self.send("player attacked");
        }
    }
}
