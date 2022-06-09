use engine::{bot::{Bot, strategy::{dummy::DummyStrategy, DecidingStrategy}}, state::GameState, action::{Action, split_bot::SplitBot}};

mod engine;

fn main() {
    engine::run_app();
}

/** FIX YELLOW */

/** FIX GREEN */

/** FIX BLUE */

/** FIX RED */


pub fn decide(bot_pos_x: usize, bot_pos_y: usize,  game_state: &GameState) -> Result<Action, String> {
    DummyStrategy.decide(bot_pos_x, bot_pos_y, game_state)
}
