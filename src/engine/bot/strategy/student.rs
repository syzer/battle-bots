use super::super::super::{
    action::{split_bot::SplitBot, Action},
    state::GameState,
};

use super::super::strategy::DecidingStrategy;

#[derive(Clone, Copy)]
pub struct StudentStrategy;

impl DecidingStrategy for StudentStrategy {
    fn decide(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &GameState) -> Result<Action, String> {
        crate::decide(bot_pos_x, bot_pos_y, game_state)
    }
}
