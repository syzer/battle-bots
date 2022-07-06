use crate::engine::{
    action::{move_bot::MoveBot, rotate_chainsaw::RotateChainsaw, Action},
    state::{GameCell, GameState, Position},
    utils::direction::{Direction, Rotation},
};

pub fn decide(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Result<Action, String> {
    /*     if let Some((x, y)) = should_attack(bot_pos_x, bot_pos_y, game_state) {
           let direction =
               adjacent_positions_to_direction(bot_pos_x, bot_pos_y, x, y).expect("Bad position");

           return Ok(Action::Attack(Attack {
               attacking_direction: direction,
           }));
       }
       if let Some(direction) = should_move_towards_enemy(bot_pos_x, bot_pos_y, game_state) {
           return Ok(Action::MoveBot(MoveBot {
               move_direction: direction,
           }));
       }
    */
    return Ok(Action::RotateChainsaw(RotateChainsaw(Rotation::Clockwise)));

    Err("Nothing to do".into())
}
/* 
pub fn is_bot(game_state: GameState, position: Position) -> bool {
    match game_cell {
        GameCell::Bot(_) => true,
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

pub fn should_move_towards_enemy(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<Direction> {
    if let Some((x, y)) = get_closest_enemy(bot_pos_x, bot_pos_y, game_state) {
        if let Ok(next_move) = next_move_in_path(bot_pos_x, bot_pos_y, x, y, game_state) {
            return Some(next_move);
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

    let (to_pos_x, to_pos_y) = moves
        .first()
        .ok_or(String::from("No moves to the chosen path"))?
        .clone();

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

pub fn distance(from_pos_x: usize, from_pos_y: usize, to_pos_x: usize, to_pos_y: usize) -> usize {
    let x_distance = absolute(to_pos_x as isize - from_pos_x as isize);
    let y_distance = absolute(to_pos_y as isize - from_pos_y as isize);

    x_distance + y_distance
}

pub fn get_closest_enemy(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &GameState,
) -> Option<(usize, usize)> {
    let mut closest_enemy: Option<(usize, usize)> = None;

    for x in 0..game_state.map_width {
        for y in 0..game_state.map_height {
            if x != bot_pos_x || y != bot_pos_y {
                if is_bot(game_state.map[x][y]) {
                    match closest_enemy {
                        Some((closest_x, closest_y))
                            if distance(closest_x, closest_y, bot_pos_x, bot_pos_y)
                                < distance(x, y, bot_pos_x, bot_pos_y) => {}
                        _ => closest_enemy = Some((x, y)),
                    };
                }
            }
        }
    }

    closest_enemy
}

// Return a vector of the adjacent positions to the given one, in the form of (x, y) tuples
// Careful! Don't return invalid positions (negative coordinates, or coordinates that exceed the map size)
pub fn get_adjacent_positions(x: usize, y: usize, game_state: &GameState) -> Vec<Position> {
    let mut positions = vec![];

    if x > 0 {
        positions.push(Position { x: x - 1, y });
    }
    if x < game_state.map_width - 1 {
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

pub fn should_attack(
    bot_pos_x: usize,
    bot_pos_y: usize,
    game_state: &Battle,
) -> Option<(usize, usize)> {
    let positions = get_adjacent_positions(bot_pos_x, bot_pos_y, game_state);

    for (x, y) in positions {
        if is_bot(game_state.map[x][y]) {
            return Some((x, y));
        }
    }

    None
} */

// Return whether we should gather a resource that's in an adjacent position to the bot's
// Return Some((resource_pos_x, resource_pos_y)) if we should gather the resource at that position
// Return None if we should not gather any resource around us
//
// Extra credit: only gather the resource if the bot has less than 9 energy
