use ruscii::terminal::Color;

use super::state::BOTS_STARTING_ENERGY;

use self::strategy::{dummy::DummyStrategy, BotStrategy, student::StudentStrategy};

pub mod strategy;

#[derive(Clone, Copy)]
pub struct Bot {
    pub energy: usize,
    pub color: Color,
    pub strategy: BotStrategy,
}

impl Bot {
    pub fn new_dummy(color: Color) -> Bot {
        Bot {
            color,
            energy: BOTS_STARTING_ENERGY,
            strategy: BotStrategy::Dummy(DummyStrategy),
        }
    }
    pub fn new_student(color: Color) -> Bot {
        Bot {
            color,
            energy: BOTS_STARTING_ENERGY,
            strategy: BotStrategy::Student(StudentStrategy),
        }
    }
}
