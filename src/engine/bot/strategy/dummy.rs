use super::super::super::{
    action::{split_bot::SplitBot, Action},
    state::GameState,
};

use super::DecidingStrategy;

#[derive(Clone, Copy)]
pub struct DummyStrategy;

impl DecidingStrategy for DummyStrategy {
    fn decide(&self, bot_pos_x: usize, bot_pos_y: usize,  game_state: &GameState) -> Action {
        Action::SplitBot(SplitBot)
    }
}
