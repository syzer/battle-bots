use ruscii::terminal::Color;

use super::{
    action::Action,
    state::{GameState, BOTS_STARTING_ENERGY},
    utils::direction::Direction,
};

pub type BotStrategy = fn(usize, usize, &GameState) -> Result<Action, String>;

pub struct ColorConfig {
    pub color: Color,
    pub number_of_bots: usize,
    pub strategy: BotStrategy,
}

#[derive(Clone, Copy)]
pub struct Bot {
    pub energy: usize,
    pub color: Color,
    pub chainsaw_direction: Direction,
    pub shield_direction: Direction,
}

impl Bot {
    pub fn new(color: Color) -> Bot {
        Bot {
            energy: BOTS_STARTING_ENERGY,
            color,
            shield_direction: Direction::Up,
            chainsaw_direction: Direction::Up,
        }
    }
}
