use super::state::{Battle, GameState};

use self::{
    gather_resource::GatherResource, move_bot::MoveBot, rotate_chainsaw::RotateChainsaw,
    rotate_shield::RotateShield,
};

pub mod gather_resource;
pub mod move_bot;
pub mod rotate_chainsaw;
pub mod rotate_shield;

pub trait ExecutableAction {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: GameState) -> GameState;
}

pub enum Action {
    GatherResource(GatherResource),
    MoveBot(MoveBot),
    RotateShield(RotateShield),
    RotateChainsaw(RotateChainsaw),
}

impl ExecutableAction for Action {
    fn execute(&self, bot_pos_x: usize, bot_pos_y: usize, game_state: GameState) -> GameState {
        match self {
            Action::GatherResource(gather_resource) => {
                gather_resource.execute(bot_pos_x, bot_pos_y, game_state)
            }
            Action::MoveBot(move_bot) => move_bot.execute(bot_pos_x, bot_pos_y, game_state),
            Action::RotateShield(rotate_shield) => {
                rotate_shield.execute(bot_pos_x, bot_pos_y, game_state)
            }
            Action::RotateChainsaw(rotate_chainsaw) => {
                rotate_chainsaw.execute(bot_pos_x, bot_pos_y, game_state)
            }
        }
    }
}
