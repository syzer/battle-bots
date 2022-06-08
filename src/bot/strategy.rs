use crate::{action::Action, state::GameState};

use super::dummy::DummyStrategy;

pub trait DecidingStrategy: Clone {
    fn decide(&self, game_state: &GameState) -> Action;
}

#[derive(Clone, Copy)]
pub enum BotStrategy {
    Dummy(DummyStrategy),
}

impl DecidingStrategy for BotStrategy {
    fn decide(&self, game_state: &GameState) -> Action {
        match self {
            BotStrategy::Dummy(d) => d.decide(game_state),
        }
    }
}
