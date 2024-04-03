use super::unit::Unit;

pub trait Creature {
    fn new() -> Self
    where
        Self: Sized;
    fn get_unit(&self) -> &Unit;
}

pub struct Squirrel {
    pub unit: Unit,
}

impl Creature for Squirrel {
    fn new() -> Self {
        let mut unit = Unit::new();
        unit.name = String::from("Squirrel");
        Squirrel { unit }
    }

    fn get_unit(&self) -> &Unit {
        &self.unit
    }
}
