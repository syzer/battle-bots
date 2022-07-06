use crate::engine::state::{from_matrix, state_to_matrix, GameState, BOTS_STARTING_ENERGY};

use super::super::{
    state::{Battle, GameCell},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct GatherResource {
    pub gathering_direction: Direction,
}

impl ExecutableAction for GatherResource {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, state: GameState) -> GameState {
        let (gathering_position_x, gathering_position_y) = self
            .gathering_direction
            .compute_position(bot_pos_x, bot_pos_y);

        let mut map = state_to_matrix(state);

        if let GameCell::Bot(mut bot) = map[bot_pos_x][bot_pos_y] {
            if let GameCell::Resource(resource) = map[gathering_position_x][gathering_position_y] {
                bot.energy += resource.energy_gain;

                if bot.energy >= BOTS_STARTING_ENERGY {
                    bot.energy = BOTS_STARTING_ENERGY;
                }

                map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);
                map[gathering_position_x][gathering_position_y] = GameCell::Empty;
            }
        }

        from_matrix(map)
    }
}
