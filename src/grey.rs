#[macro_use]
// extern crate serde_json;
// extern crate serde;

use battle_bots_engine::*;
// use std::num::Wrapping;
use serde_json::json;

// use core::fmt::{Debug, Formatter};
// use std::fmt;
// use serde::Serialize;



/** 
 * The grey bot is broken! It's using all the functions below, but they seem not to be implemented correctly
 * 
 * Please help us fix the bot!
 * 
 * Instructions
 * =============
 * 
 * Implement all the functions below
 * Run a battle (`cargo run`) after they have been implemented to test that the grey bot works again
 */


// Returns the position that's adjacent to the given one in the given direction, in the form (x, y)
// eg. adjacent_position_in_direction(4, 5, Direction::Down) == (4, 6)
pub fn adjacent_position_in_direction(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    if x == 0 || y == 0 {
        return (1, 1);
    }
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}




// Returns whether there is a bot in the given position
pub fn is_bot(game_state: &GameState, position: &Position) -> bool {
    game_state.bots
        .iter()
        .any(|(pos, _)| pos.x == position.x 
            && pos.y == position.y)
}

// Returns the shortest way to rotate the "from" direction to get the "to" direction
// Assumes that from and to are not equal
// eg. shortest_rotation(Direction::Up, Direction::Right) == Rotation::Clockwise
pub fn shortest_rotation(from: &Direction, to: &Direction) -> Rotation {
    match (from, to) {
        (Direction::Up, Direction::Left) => Rotation::Counterclockwise,
        (Direction::Left, Direction::Down) => Rotation::Counterclockwise,
        (Direction::Down, Direction::Right) => Rotation::Counterclockwise,
        (Direction::Right, Direction::Up) => Rotation::Counterclockwise,
        _ => Rotation::Clockwise,
    }
}

// Rotate the given direction with the given rotation
// eg. rotate_direction(Direction::Up, Rotation::Clockwise) == Direction::Right
pub fn rotate_direction(direction: &Direction, rotation: &Rotation) -> Direction {
    match (direction, rotation) {
        (Direction::Up, Rotation::Clockwise) => Direction::Right,
        (Direction::Up, Rotation::Counterclockwise) => Direction::Left,
        (Direction::Right, Rotation::Clockwise) => Direction::Down,
        (Direction::Right, Rotation::Counterclockwise) => Direction::Up,
        (Direction::Down, Rotation::Clockwise) => Direction::Left,
        (Direction::Down, Rotation::Counterclockwise) => Direction::Right,
        (Direction::Left, Rotation::Clockwise) => Direction::Up,
        (Direction::Left, Rotation::Counterclockwise) => Direction::Down,
        // _ => Direction::Up,
    }
}
