use ruscii::terminal::Color;

use super::state::BOTS_STARTING_ENERGY;

use self::strategy::{
    blue::BlueStrategy, dummy::DummyStrategy, green::GreenStrategy, red::RedStrategy,
    yellow::YellowStrategy, BotStrategy,
};

pub mod strategy;

#[derive(Clone, Copy)]
pub struct Bot {
    pub energy: usize,
    pub color: Color,
    pub strategy: BotStrategy,
    pub tiredness: usize,
}

impl Bot {
    pub fn new_dummy(color: Color) -> Bot {
        Bot {
            color,
            energy: BOTS_STARTING_ENERGY,
            strategy: BotStrategy::Dummy(DummyStrategy),
            tiredness: 0,
        }
    }

    pub fn new(color: Color) -> Bot {
        Bot {
            color,
            energy: BOTS_STARTING_ENERGY,
            strategy: Self::strategy(color),
            tiredness: 0,
        }
    }

    fn strategy(color: Color) -> BotStrategy {
        match color {
            Color::Blue => BotStrategy::Blue(BlueStrategy),
            Color::Yellow => BotStrategy::Yellow(YellowStrategy),
            Color::Green => BotStrategy::Green(GreenStrategy),
            Color::Red => BotStrategy::Red(RedStrategy),
            _ => BotStrategy::Dummy(DummyStrategy),
        }
    }
}
