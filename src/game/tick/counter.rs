pub(crate) struct TickCounter {
    tick: i32,
    cooldown: i32,
}

impl TickCounter {
    pub fn new(cooldown: i32) -> TickCounter {
        TickCounter { tick: 0, cooldown }
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

    fn ready(&self) -> bool {
        self.tick >= self.cooldown
    }
}
