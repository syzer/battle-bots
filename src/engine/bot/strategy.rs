use super::super::{action::Action, state::GameState};

pub mod dummy;
pub mod student;

use dummy::DummyStrategy;
use student::StudentStrategy;

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
    Student(StudentStrategy),
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
            BotStrategy::Student(s) => s.decide(bot_pos_x, bot_pos_y, game_state),
        }
    }
}
