use crate::{
    state::{GameCell, GameState},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct GatherResource {
    gathering_direction: Direction,
}

impl ExecutableAction for GatherResource {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        let (gathering_position_x, gathering_position_y) = self
            .gathering_direction
            .compute_position(bot_pos_x, bot_pos_y);

        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            if let GameCell::Resource(resource) =
                game_state.map[gathering_position_x][gathering_position_y]
            {
                bot.energy += resource.energy_gain;
                game_state.map[gathering_position_x][gathering_position_y] = GameCell::Empty;
            }
        }
    }
}
