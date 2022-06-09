/** FIX YELLOW */

pub fn absolute(n: isize) -> usize {
    3
}

pub fn is_adjacent_position(
    from_pos_x: usize,
    from_pos_y: usize,
    to_pos_x: usize,
    to_pos_y: usize,
) -> bool {
    false
}

pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
    100
}

// For loop

// Arrays

// Move semantics

/** FIX GREEN */

// Return whether the given game_cell is a Bot or not
//
// GameCell is an enum that looks like this:
// ```rust
//  enum GameCell {
//      Empty,
//      Bot(Bot),
//      Resource(Resource),
//  }
// ```
pub fn is_bot(game_cell: GameCell) -> bool {
    false
}

// Return whether the given game_cell is a Resource or not
//
// GameCell is an enum that looks like this:
// ```rust
//  enum GameCell {
//      Empty,
//      Bot(Bot),
//      Resource(Resource),
//  }
// ```
pub fn is_resource(game_cell: GameCell) -> bool {
    false
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn get_adjacent_positions(x: usize, y: usize, game_state: &GameState) -> Vec<(usize, usize)> {
    vec![]
}

// Return whether we should gather a resource that's in an adjacent position to the bot's
// Return Some((resource_pos_x, resource_pos_y)) if we should gather the resource at that position
// Return None if we should not gather any resource around us
//
// Extra credit: only gather the resource if the bot has less than 9 energy
pub fn should_gather_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    None
}

/** FIX BLUE */

pub fn adjacent_positions_to_direction(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<Direction, String> {
    Ok(Direction::Down)
}

fn should_attack(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    None
}

fn get_closest_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    None
}

/** FIX RED */

pub fn decide(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Result<Action, String> {
    DummyStrategy.decide(bot_pos_x, bot_pos_y, game_state)
}

/** Don't change this last part  */
use engine::{
    action::Action,
    bot::strategy::{dummy::DummyStrategy, DecidingStrategy},
    state::{GameCell, GameState},
    utils::direction::Direction,
};

mod engine;

fn main() {
    engine::run_app();
}
