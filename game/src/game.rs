use super::unit::unit::{Unit, Moveable, Direction};

pub trait GameEvent {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn stop(&mut self);
}

pub struct Game {
    pub playable: Unit,
    playable_direction: Option<Direction>,
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
        let mut nemo = Unit::new();
        nemo.set_boundary((800, 600), true);
        Game {
            playable: nemo,
            playable_direction: Option::None,
        }
    }

    pub fn tick(&mut self) {
        self.move_playable();
        self.playable.tick();
    }

    fn move_playable(&mut self) {
        if let Some(ref direction) = self.playable_direction {
            self.playable.move_shape(direction);
        } else {
            return;
        }
    }
}
