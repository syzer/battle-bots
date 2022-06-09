use crate::engine::{
    action::{attack::Attack, gather_resource::GatherResource, move_bot::MoveBot},
    state::GameCell,
    utils::direction::Direction,
};

use super::{super::super::{
    action::{split_bot::SplitBot, Action},
    state::GameState,
}, dummy::random_move};

use super::super::strategy::DecidingStrategy;

#[derive(Clone, Copy)]
pub struct GreenStrategy;

impl DecidingStrategy for GreenStrategy {
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

        if let Some((x, y)) = crate::should_gather_resource(bot_pos_x, bot_pos_y, game_state) {
            return Ok(Action::GatherResource(GatherResource {
                gathering_direction: adjacent_positions_to_direction(bot_pos_x, bot_pos_y, x, y)?,
            }));
        }

        if let Some(direction) = should_move_towards_resource(bot_pos_x, bot_pos_y, game_state) {
            return Ok(Action::MoveBot(MoveBot {
                move_direction: direction,
            }));
        }

        return Ok(Action::MoveBot(MoveBot {
            move_direction: random_move(),
        }));
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

pub fn get_attacking_force(
    bot_pos_x: usize,
    bot_pos_y: usize,
    attacking_direction: &Direction,
    game_state: &GameState,
) -> usize {
    2
}

fn get_closest_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let mut closest_enemy: Option<(usize, usize)> = None;

    for x in 0..game_state.map.len() {
        for y in 0..game_state.map[0].len() {
            if crate::is_resource(game_state.map[x][y]) {
                match closest_enemy {
                    Some((closest_x, closest_y))
                        if super::dummy::distance(closest_x, closest_y, bot_pos_x, bot_pos_y)
                            < super::dummy::distance(x, y, bot_pos_x, bot_pos_y) => {}
                    _ => closest_enemy = Some((x, y)),
                };
            }
        }
    }

    closest_enemy
}

pub fn should_move_towards_resource(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<Direction> {
    if let Some((x, y)) = get_closest_resource(bot_pos_x, bot_pos_y, game_state) {
        if super::dummy::distance(bot_pos_x, bot_pos_y, x, y) < 5 {
            if let Ok(next_move) = next_move_in_path(bot_pos_x, bot_pos_y, x, y, game_state) {
                return Some(next_move);
            }
        }
    }

    None
}

fn should_attack(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let positions = crate::get_adjacent_positions(bot_pos_x, bot_pos_y, game_state);

    for (x, y) in positions {
        if crate::is_bot(game_state.map[x][y]) {
            return Some((x, y));
        }
    }

    None
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

        let adjacents = crate::get_adjacent_positions(current_x, current_y, game_state);

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
