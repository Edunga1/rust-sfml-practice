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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() {
        let mut counter = TickCounter::new(0, 1);
        counter.tick();
        assert_eq!(counter.tick, 1);
    }

    #[test]
    fn test_reset() {
        let mut counter = TickCounter::new(1, 1);
        let result = counter.reset();
        assert_eq!(result, true);
        assert_eq!(counter.tick, 0);
    }

    #[test]
    fn test_force_reset() {
        let mut counter = TickCounter::new(1, 1);
        counter.force_reset();
        assert_eq!(counter.tick, 0);
    }

    #[test]
    fn test_ready() {
        let counter = TickCounter::new(1, 1);
        let result = counter.ready();
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_ready() {
        let counter = TickCounter::new(0, 1);
        let result = counter.ready();
        assert_eq!(result, false);
    }
}
