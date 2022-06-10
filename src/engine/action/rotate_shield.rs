use super::super::{
    state::{GameCell, GameState},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct RotateShield {
    pub final_direction: Direction,
}

impl ExecutableAction for RotateShield {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        let (shield_position_x, shield_position_y) =
            self.final_direction.compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            if let GameCell::Empty = &game_state.map[shield_position_x][shield_position_y] {
                bot.shield_direction = self.final_direction;

                game_state.map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
            }
        }
    }
}
