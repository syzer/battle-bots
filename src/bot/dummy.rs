use crate::{
    action::{split_bot::SplitBot, Action},
    state::GameState,
};

use super::strategy::DecidingStrategy;

#[derive(Clone, Copy)]
pub struct DummyStrategy;

impl DecidingStrategy for DummyStrategy {
    fn decide(&self, game_state: &GameState) -> Action {
        Action::SplitBot(SplitBot)
    }
}
