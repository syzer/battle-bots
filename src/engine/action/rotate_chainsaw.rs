use crate::engine::{
    state::{from_matrix, state_to_matrix, GameState},
    utils::direction::Rotation,
};

use super::super::state::{Battle, GameCell};

use super::ExecutableAction;

pub struct RotateChainsaw(pub Rotation);

impl ExecutableAction for RotateChainsaw {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, state: GameState) -> GameState {
        let mut map = state_to_matrix(state);

        if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
            bot.chainsaw_direction = bot.chainsaw_direction.rotate(self.0);

            map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
        }
        from_matrix(map)
    }
}
