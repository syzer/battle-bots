use battle_bots_engine::*;
use std::cmp::Ordering;

use crate::yellow::{is_position_inside_map_bounds, distance};

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

pub fn dec (num: usize) -> usize {
  match num {
    0 => 0,
    _ => num - 1
  }  
}

pub fn inc (num: usize, max: usize) -> usize {
  if num <= max {
    num + 1
  } else {
    max
  }
  // match num {
    // max => max,
    // _ => num + 1
  // }
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn valid_adjacent_positions(game_state: &GameState, position: &Position) -> Vec<Position> {
  let up = Position { x: position.x, y: dec(position.y)};
  let down = Position { x: position.x, y: inc(position.y, game_state.map_height) };
  let right: Position = Position { x: inc(position.x, game_state.map_width), y: position.y };
  let left: Position = Position { x: dec(position.x), y: position.y };

  vec![up, right, down, left]
    .into_iter()
    .filter(|pos| pos.x != position.x || pos.y != position.y)
    .filter(|p| is_position_inside_map_bounds(p.x, p.y, game_state.map_width, game_state.map_height))
    .collect() 
}

// Returns the direction that the to position is relative to the from position
// eg: adjacent_positions_to_direction(Position { x: 0, y: 0 }, Position { x: 1, y: 0 }) == Direction::Right
pub fn adjacent_positions_to_direction(from: &Position, to: &Position) -> Result<Direction, String> {
  match (from, to) {
    (from, to) if from.x == to.x && from.y == to.y => Err("Not valid".into()),
    // TODO
    (from, to) if from.x == to.x && to.y > 0 && from.y == to.y - 1      => Ok(Direction::Down),
    (from, to) if from.x == to.x && from.y > 0 && from.y - 1 == to.y      => Ok(Direction::Up),
    (from, to) if from.x > 0 && from.x - 1 == to.x && from.y == to.y    => Ok(Direction::Right),
    // TODO probably not the best way to handle this
    _ => Ok(Direction::Left)
  }
}

// Returns whether there is an adjacent bot, and its position if there is one
pub fn adjacent_bot(game_state: &GameState, bot_position: &Position) -> Option<Direction> {
  let pos = valid_adjacent_positions(game_state, bot_position)
    .into_iter()
    .find(|p| bot_in_position(game_state, &p).is_some());

  match pos {
    None => return None,
    Some(p) => 
      adjacent_positions_to_direction(
        bot_position, 
        &p).ok()
  } 
}

// Returns the position of the closest enemy
pub fn get_closest_enemy(game_state: &GameState, bot_position: &Position) -> Option<Position> {
  let a = game_state.bots
        .iter()
        .min_by(|(a, _), (b, _)| 
            distance(a.x, a.y, bot_position.x, bot_position.y)
              .cmp(&distance(b.x, b.y, bot_position.x, bot_position.y)))
        .map(|(p, _)| p);
  
        
  match a {
    Some(p) => Some(p.clone()),
    None => None 
  }
}
