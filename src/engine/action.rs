use super::state::GameState;

use self::{
    attack::Attack, gather_resource::GatherResource, move_bot::MoveBot, split_bot::SplitBot,
};

pub mod attack;
pub mod gather_resource;
pub mod move_bot;
pub mod split_bot;

pub trait ExecutableAction {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> ();
}

pub enum Action {
    Attack(Attack),
    GatherResource(GatherResource),
    MoveBot(MoveBot),
    SplitBot(SplitBot),
}

impl ExecutableAction for Action {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: &mut GameState) -> () {
        match self {
            Action::Attack(attack) => attack.execute(bot_pos_x, bot_pos_y, game_state),
            Action::GatherResource(gather_resource) => {
                gather_resource.execute(bot_pos_x, bot_pos_y, game_state)
            }
            Action::MoveBot(move_bot) => move_bot.execute(bot_pos_x, bot_pos_y, game_state),
            Action::SplitBot(split_bot) => split_bot.execute(bot_pos_x, bot_pos_y, game_state),
        }
    }
}
