use super::super::{
    bot::Bot,
    state::{GameCell, GameState, MOVE_ENERGY_LOSS, SPLIT_ENERGY_LOSS},
    utils::direction::Direction,
};

use super::ExecutableAction;

pub struct SplitBot;

impl ExecutableAction for SplitBot {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        if let GameCell::Bot(mut bot) = game_state.map[bot_pos_x][bot_pos_y] {
            bot.energy -= SPLIT_ENERGY_LOSS;
            bot.energy = bot.energy / 2;

            game_state.map[bot_pos_x][bot_pos_y] = GameCell::Bot(bot);

            let (up_bot_x, up_bot_y) = Direction::Up.compute_position(bot_pos_x, bot_pos_y);
            let (down_bot_x, down_bot_y) = Direction::Down.compute_position(bot_pos_x, bot_pos_y);
            let (left_bot_x, left_bot_y) = Direction::Left.compute_position(bot_pos_x, bot_pos_y);
            let (right_bot_x, right_bot_y) =
                Direction::Right.compute_position(bot_pos_x, bot_pos_y);

            let new_bot = Bot {
                strategy: bot.strategy.clone(),
                color: bot.color,
                energy: bot.energy,
            };

            if let GameCell::Empty = game_state.map[up_bot_x][up_bot_y] {
                game_state.map[up_bot_x][up_bot_y] = GameCell::Bot(new_bot);
            } else if let GameCell::Empty = game_state.map[down_bot_x][down_bot_y] {
                game_state.map[down_bot_x][down_bot_y] = GameCell::Bot(new_bot);
            } else if let GameCell::Empty = game_state.map[left_bot_x][left_bot_y] {
                game_state.map[left_bot_x][left_bot_y] = GameCell::Bot(new_bot);
            } else if let GameCell::Empty = game_state.map[right_bot_x][right_bot_y] {
                game_state.map[right_bot_x][right_bot_y] = GameCell::Bot(new_bot);
            }
        }
    }
}
