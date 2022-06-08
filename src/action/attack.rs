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
            bot.energy -= self.force / 10;
            if let GameCell::Bot(mut attacked_bot) =
                game_state.map[attacked_position_x][attacked_position_y]
            {
                let original_energy = attacked_bot.energy;
                attacked_bot.energy -= self.force;
                if attacked_bot.energy <= 0 {
                    bot.energy += original_energy;
                }
            }
        }
    }
}
