use super::super::{action::Action, state::GameState};

pub mod blue;
pub mod dummy;
pub mod green;
pub mod red;
pub mod yellow;

use blue::BlueStrategy;
use dummy::DummyStrategy;
use green::GreenStrategy;
use red::RedStrategy;
use yellow::YellowStrategy;

pub trait DecidingStrategy: Clone {
    fn decide(
        &self,
        bot_pos_x: usize,
        bot_pos_y: usize,
        game_state: &GameState,
    ) -> Result<Action, String>;
}

#[derive(Clone, Copy)]
pub enum BotStrategy {
    Dummy(DummyStrategy),
    Red(RedStrategy),
    Blue(BlueStrategy),
    Green(GreenStrategy),
    Yellow(YellowStrategy),
}

impl DecidingStrategy for BotStrategy {
    fn decide(
        &self,
        bot_pos_x: usize,
        bot_pos_y: usize,
        game_state: &GameState,
    ) -> Result<Action, String> {

        match self {
            BotStrategy::Dummy(d) => d.decide(bot_pos_x, bot_pos_y, game_state),
            BotStrategy::Red(s) => s.decide(bot_pos_x, bot_pos_y, game_state),
            BotStrategy::Green(s) => s.decide(bot_pos_x, bot_pos_y, game_state),
            BotStrategy::Blue(s) => s.decide(bot_pos_x, bot_pos_y, game_state),
            BotStrategy::Yellow(s) => s.decide(bot_pos_x, bot_pos_y, game_state),
        }
    }
}
