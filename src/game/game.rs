use sfml::system::Vector2f;

use crate::shape::nemo::{Nemo, Moveable};

pub trait GameEvent {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn stop(&mut self);
}

pub struct Game<'a> {
    pub playable: Nemo<'a>,
    playable_direction: Vector2f,
}

impl GameEvent for Game<'_> {
    fn move_left(&mut self) {
        self.playable_direction = (-Game::SPEED, 0.).into();
    }

    fn move_right(&mut self) {
        self.playable_direction = (Game::SPEED, 0.).into();
    }

    fn move_up(&mut self) {
        self.playable_direction = (0., -Game::SPEED).into();
    }

    fn move_down(&mut self) {
        self.playable_direction = (0., Game::SPEED).into();
    }

    fn stop(&mut self) {
        self.playable_direction = (0., 0.).into();
    }
}

impl Game<'_> {
    const SPEED: f32 = 0.5;

    pub fn new() -> Game<'static> {
        let mut nemo = Nemo::new();
        nemo.set_boundary((800., 600.), true);
        Game {
            playable: nemo,
            playable_direction: (0., 0.).into(),
        }
    }

    pub fn tick(&mut self) {
        self.move_playable();
    }

    fn move_playable(&mut self) {
        self.playable.move_shape(self.playable_direction);
    }
}
