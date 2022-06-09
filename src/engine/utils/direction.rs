use super::super::state::{MAP_HEIGHT, MAP_WIDTH};

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn compute_position(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up if y + 1 < MAP_HEIGHT => (x, y + 1),
            Direction::Down if (y as isize) - 1 >= 0 => (x, y - 1),
            Direction::Right if x + 1 < MAP_WIDTH => (x + 1, y),
            Direction::Left if (x as isize) - 1 >= 0 => (x - 1, y),
            _ => (x, y),
        }
    }
}
