use crate::engine::state::{from_matrix, state_to_matrix, GameState};

use super::super::{
    state::{Battle, GameCell},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct MoveBot {
    pub move_direction: Direction,
}

impl ExecutableAction for MoveBot {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, state: GameState) -> GameState {
        let (final_position_x, final_position_y) =
            self.move_direction.compute_position(bot_pos_x, bot_pos_y);
        let mut map = state_to_matrix(state);

        if let GameCell::Bot(bot) = map[bot_pos_x][bot_pos_y] {
            if let GameCell::Empty = map[final_position_x][final_position_y] {
                map[final_position_x][final_position_y] = GameCell::Bot(bot);
                map[bot_pos_x][bot_pos_y] = GameCell::Empty;
            }
        }
        from_matrix(map)
    }
}
