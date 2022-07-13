use battle_bots_engine::*;
use std::cmp::Ordering;

use crate::yellow::is_position_inside_map_bounds;

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
   game_state.bots
      .iter()
      .find(|(pos, bot)| pos.x == position.x && pos.y == position.y)
      .map(|(_, bot)| bot.clone())
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn valid_adjacent_positions(game_state: &GameState, position: &Position) -> Vec<Position> {
  let up = Position { x: position.x, y: position.y - 1 };
  let down = Position { x: position.x, y: position.y + 1 };
  let right: Position = Position { x: position.x + 1, y: position.y };
  let left: Position = Position { x: position.x - 1, y: position.y };

  vec![up, right, down, left]
    .into_iter()
    .filter(|p| is_position_inside_map_bounds(p.x, p.y, game_state.map_width, game_state.map_height))
    .collect() 
}

// Returns the direction that the to position is relative to the from position
// eg: adjacent_positions_to_direction(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }) == Direction::Right
pub fn adjacent_positions_to_direction(from: &Position, to: &Position) -> Result<Direction, String> {
  match (from, to) {
    (from, to) if from.x == to.x && from.y == to.y - 1      => Ok(Direction::Up),
    (from, to) if from.x == to.x && from.y - 1 == to.y      => Ok(Direction::Down),
    (from, to) if from.x - 1 == to.x && from.y == to.y      => Ok(Direction::Left),

    _ => Ok(Direction::Right)
  }
  // Err(String::from("beep beep bop!"))
}

// Returns whether there is an adjacent bot, and its position if there is one
pub fn adjacent_bot(game_state: &GameState, bot_position: &Position) -> Option<Direction> {
  let pos = valid_adjacent_positions(game_state, bot_position)
    .into_iter()
    .find(|p| bot_in_position(game_state, &p).is_some());

  if let pos = None {
    return None;
  }
  Some(adjacent_positions_to_direction(bot_position, &p).unwrap())
}

// Returns the position of the closest enemy
pub fn get_closest_enemy(game_state: &GameState, bot_position: &Position) -> Option<Position> {
  // game_state.bots.into_iter().sort
  let a = game_state.bots
        .into_iter()
        // .enumerate()
        .max_by(|(a, _), (b, _)| a.x.cmp(&b.x))
        // .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(p, _)| p);
  
  // if let a = None {
  //   return None;
  // }
  return a;
  // None
}
