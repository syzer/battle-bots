use engine::{bot::Bot, state::GameState, action::{Action, split_bot::SplitBot}};

mod engine;

fn main() {
    engine::run_app();
}

pub fn decide(bot_pos_x: usize, bot_pos_y: usize,  game_state: &GameState) -> Action {
    Action::SplitBot(SplitBot)
}
