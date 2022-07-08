use battle_bots_engine::*;

/**
 * The red bot is broken! It's using all the functions below, but they seem not to be implemented correctly
 *
 * Please help us fix the bot!
 *
 * Instructions
 * =============
 *
 * Implement all the functions below
 * Run a battle (`cargo run`) after they have been implemented to test that the red bot works again
 */

// Returns a bot if there is one in the given position
pub fn bot_in_position(game_state: &GameState, position: &Position) -> Option<Bot> {
    None
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn valid_adjacent_positions(game_state: &GameState, position: &Position) -> Vec<Position> {
    vec![]
}

// Returns the direction that the to position is relative to the from position
// eg: adjacent_positions_to_direction(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }) == Direction::Right
pub fn adjacent_positions_to_direction(from: &Position, to: &Position) -> Result<Direction, String> {
  Err(String::from("beep beep bop!"))
}

// Returns whether there is an adjacent bot, and its position if there is one
pub fn adjacent_bot(game_state: &GameState,bot_position: &Position,) -> Option<Direction> {
    None
}

// Returns the position of the closest enemy
pub fn get_closest_enemy(game_state: &GameState,bot_position: &Position,) -> Option<Position> {
  None
}
