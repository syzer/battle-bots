use ruscii::terminal::Color;

use crate::state::BOTS_STARTING_ENERGY;

use self::{strategy::BotStrategy, dummy::DummyStrategy};

pub mod dummy;
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
}
