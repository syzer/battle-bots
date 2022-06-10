use crate::blue::adjacent_positions_to_direction;

use super::super::{
    state::{GameCell, GameState},
    utils::direction::Direction,
};

use super::ExecutableAction;

const ATTACK_FORCE: usize = 2;

pub struct Attack {
    pub attacking_direction: Direction,
}

impl ExecutableAction for Attack {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        let (attacked_position_x, attacked_position_y) = self
            .attacking_direction
            .compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            // bot.energy -= self.force / 10;
            if let GameCell::Bot(mut attacked_bot) =
                &game_state.map[attacked_position_x][attacked_position_y]
            {
                if let Ok(direction) = adjacent_positions_to_direction(
                    attacked_position_x,
                    attacked_position_y,
                    bot_pos_x,
                    bot_pos_y,
                ) {
                    if direction != attacked_bot.shield_direction {
                        if attacked_bot.energy <= ATTACK_FORCE {
                            attacked_bot.energy = 0;
                        } else {
                            attacked_bot.energy -= ATTACK_FORCE;
                        }
                    }

                    game_state.map[attacked_position_x][attacked_position_y] =
                        GameCell::Bot(attacked_bot);
                }
            }
        }
    }
}
