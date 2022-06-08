use ruscii::terminal::Color;

use crate::{
    action::{move_bot::MoveBot, Action},
    state::GameState,
    utils::direction::Direction,
};

#[derive(Clone, Copy)]
pub struct Bot {
    pub health: usize,
    pub color: Color
}

impl Bot {
    pub fn decide(&self, game_state: &GameState) -> Action {
        Action::MoveBot(MoveBot {
            move_direction: Direction::Up,
        })
    }
}
