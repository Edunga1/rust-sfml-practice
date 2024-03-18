use super::unit::unit::{Unit, Moveable, Direction};

pub trait GameEvent {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn stop(&mut self);
}

pub struct Game<'a> {
    pub playable: Unit<'a>,
    playable_direction: Option<Direction>,
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

impl Game<'_> {
    pub fn new() -> Game<'static> {
        let mut nemo = Unit::new();
        nemo.set_boundary((800., 600.), true);
        Game {
            playable: nemo,
            playable_direction: Option::None,
        }
    }

    pub fn tick(&mut self) {
        self.move_playable();
    }

    fn move_playable(&mut self) {
        if self.playable_direction.is_none() {
            return;
        }

        let direction = self.playable_direction.take().unwrap();
        self.playable.move_shape(direction);
    }
}
