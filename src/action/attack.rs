use crate::{
    state::{GameCell, GameState},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct Attack {
    attacking_direction: Direction,
    force: usize,
}

impl ExecutableAction for Attack {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        let (attacked_position_x, attacked_position_y) = self
            .attacking_direction
            .compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            bot.health -= self.force / 10;
        }
        if let GameCell::Bot(mut bot) = game_state.map[attacked_position_x][attacked_position_y] {
            bot.health -= self.force;
        }
    }
}
