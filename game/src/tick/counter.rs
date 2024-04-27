#[derive(Debug)]
pub(crate) struct TickCounter {
    tick: i32,
    cooldown: i32,
}

impl TickCounter {
    pub fn new(tick: i32, cooldown: i32) -> TickCounter {
        TickCounter { tick, cooldown }
    }

    pub fn tick(&mut self) {
        if self.ready() {
            return;
        }
        self.tick += 1;
    }

    pub fn reset(&mut self) -> bool {
        if self.ready() {
            self.tick = 0;
            return true;
        }
        false
    }

    pub fn force_reset(&mut self) {
        self.tick = 0;
    }

    fn ready(&self) -> bool {
        self.tick >= self.cooldown
    }
}

impl From<i32> for TickCounter {
    fn from(cooldown: i32) -> TickCounter {
        TickCounter::new(0, cooldown)
    }
}

impl From<(i32, i32)> for TickCounter {
    fn from(tuple: (i32, i32)) -> TickCounter {
        TickCounter::new(tuple.0, tuple.1)
    }
}
