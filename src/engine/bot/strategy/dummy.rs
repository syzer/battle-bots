use crate::engine::{
    action::{attack::Attack, gather_resource::GatherResource, move_bot::MoveBot},
    state::GameCell,
    utils::direction::Direction,
};

use super::super::super::{action::Action, state::GameState};

use super::DecidingStrategy;

#[derive(Clone, Copy)]
pub struct DummyStrategy;

impl DecidingStrategy for DummyStrategy {
    fn decide(
        &self,
        bot_pos_x: usize,
        bot_pos_y: usize,
        game_state: &GameState,
    ) -> Result<Action, String> {
        if let Some((x, y)) = should_attack(bot_pos_x, bot_pos_y, game_state) {
            let direction =
                adjacent_positions_to_direction(bot_pos_x, bot_pos_y, x, y).expect("Bad position");

            let force = get_attacking_force(bot_pos_x, bot_pos_y, &direction, game_state);

            return Ok(Action::Attack(Attack {
                attacking_direction: direction,
                force,
            }));
        }

        if let Some((x, y)) = should_gather_resource(bot_pos_x, bot_pos_y, game_state) {
            return Ok(Action::GatherResource(GatherResource {
                gathering_direction: adjacent_positions_to_direction(bot_pos_x, bot_pos_y, x, y)?,
            }));
        }

        if let Some(direction) = should_move_towards_resource(bot_pos_x, bot_pos_y, game_state) {
            return Ok(Action::MoveBot(MoveBot {
                move_direction: direction,
            }));
        }

        Ok(Action::MoveBot(MoveBot {
            move_direction: Direction::Up,
        }))
    }
}

pub fn is_bot(game_cell: GameCell) -> bool {
    match game_cell {
        GameCell::Bot(_) => true,
        _ => false,
    }
}
pub fn is_resource(game_cell: GameCell) -> bool {
    match game_cell {
        GameCell::Resource(_) => true,
        _ => false,
    }
}

pub fn absolute(n: isize) -> usize {
    if n < 0 {
        return -n as usize;
    } else {
        return n as usize;
    }
}

pub fn adjacent_positions_to_direction(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
) -> Result<Direction, String> {
    if from_x + 1 == to_x && from_y == to_y {
        return Ok(Direction::Right);
    } else if from_x == to_x + 1 && from_y == to_y {
        return Ok(Direction::Left);
    } else if from_x == to_x && from_y + 1 == to_y {
        return Ok(Direction::Up);
    } else if from_x == to_x && from_y == to_y + 1 {
        return Ok(Direction::Down);
    }

    Err(String::from("Positions are not adjacent"))
}

pub fn should_move_towards_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<Direction> {
    if let Some((x, y)) = get_closest_resource(bot_pos_x, bot_pos_y, game_state) {
        if distance(bot_pos_x, bot_pos_y, x, y) < 100 {
            if let Ok(next_move) = next_move_in_path(bot_pos_x, bot_pos_y, x, y, game_state) {
                return Some(next_move);
            }
        }
    }

    None
}

pub fn next_move_in_path(
    from_pos_x: usize,
    from_pos_y: usize,
    to_pos_x: usize,
    to_pos_y: usize,
    game_state: &GameState,
) -> Result<Direction, String> {
    let moves = find_shortest_path(from_pos_x, from_pos_y, to_pos_x, to_pos_y, game_state)?;

    let (to_pos_x, to_pos_y) = moves[0];

    adjacent_positions_to_direction(from_pos_x, from_pos_y, to_pos_x, to_pos_y)
}

pub fn find_shortest_path(
    from_pos_x: usize,
    from_pos_y: usize,
    to_pos_x: usize,
    to_pos_y: usize,
    game_state: &GameState,
) -> Result<Vec<(usize, usize)>, String> {
    // BFS

    let mut visited = vec![vec![false; game_state.map[0].len()]; game_state.map.len()];
    let mut queue: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![];

    visited[from_pos_x][from_pos_y] = true;
    queue.push(((from_pos_x, from_pos_y), vec![]));

    while !queue.is_empty() {
        let ((current_x, current_y), path) = queue.remove(0);

        if current_x == to_pos_x && current_y == to_pos_y {
            let mut new_path = path.clone();

            new_path.push((current_x, current_y));
            new_path.remove(0);

            return Ok(new_path);
        }

        let adjacents = get_adjacent_positions(current_x, current_y, game_state);

        for (adjacent_x, adjacent_y) in adjacents {
            if !visited[adjacent_x][adjacent_y] {
                visited[adjacent_x][adjacent_y] = true;

                let mut new_path = path.clone();

                new_path.push((current_x, current_y));

                queue.push(((adjacent_x, adjacent_y), new_path));
            }
        }
    }

    Err("There is no available path".into())
}

pub fn get_attacking_force(
    bot_pos_x: usize,
    bot_pos_y: usize,
    attacking_direction: &Direction,
    game_state: &GameState,
) -> usize {
    5
}

pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
    let x_distance = absolute(to_pos_x as isize - from_pos_x as isize);
    let y_distance = absolute(to_pos_y as isize - from_pos_y as isize);

    x_distance + y_distance
}

fn get_closest_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let mut closest_enemy: Option<(usize, usize)> = None;

    for x in 0..game_state.map.len() {
        for y in 0..game_state.map[0].len() {
            if is_resource(game_state.map[x][y]) {
                match closest_enemy {
                    Some((closest_x, closest_y))
                        if distance(closest_x, closest_y, bot_pos_x, bot_pos_y)
                            < distance(x, y, bot_pos_x, bot_pos_y) => {}
                    _ => closest_enemy = Some((x, y)),
                };
            }
        }
    }

    closest_enemy
}

fn get_adjacent_positions(x: usize, y: usize, game_state: &GameState) -> Vec<(usize, usize)> {
    let mut positions = vec![];

    if x > 0 {
        positions.push((x - 1, y));
    }
    if x < game_state.map.len() - 1 {
        positions.push((x + 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if y < game_state.map[0].len() - 1 {
        positions.push((x, y + 1));
    }

    positions
}

fn should_attack(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let positions = get_adjacent_positions(bot_pos_x, bot_pos_y, game_state);

    for (x, y) in positions {
        if is_bot(game_state.map[x][y]) {
            return Some((x, y));
        }
    }

    None
}

fn should_gather_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let positions = get_adjacent_positions(bot_pos_x, bot_pos_y, game_state);

    for (x, y) in positions {
        if is_resource(game_state.map[x][y]) {
            return Some((x, y));
        }
    }

    None
}
