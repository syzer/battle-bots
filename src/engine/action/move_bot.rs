use super::super::{
    state::{GameCell, GameState, MOVE_ENERGY_LOSS},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct MoveBot {
    pub move_direction: Direction,
}

impl ExecutableAction for MoveBot {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        let (final_position_x, final_position_y) =
            self.move_direction.compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            if let GameCell::Empty = game_state.map[final_position_x][final_position_y] {
                bot.energy -= MOVE_ENERGY_LOSS;

                game_state.map[final_position_x][final_position_y] = GameCell::Bot(bot);
                game_state.map[bot_pos_x][bot_pos_y] = GameCell::Empty;
            }
        }
    }
}
