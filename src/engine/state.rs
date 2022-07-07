use rand::Rng;
use ruscii::terminal::Color;

use super::{
    bot::{Bot, BotStrategy, ColorConfig},
    resource::Resource,
};

pub const AVERAGE_RESOURCE_GENERATION_PER_TICK: usize = 2;
pub const MAX_RESOURCES: usize = 20;
pub const RESOURCE_MAX_ENERGY_GAIN: usize = 4;
pub const RESOURCE_MIN_ENERGY_GAIN: usize = 2;
pub const MAP_HEIGHT: usize = 10;
pub const MAP_WIDTH: usize = 30;
pub const BOTS_STARTING_ENERGY: usize = 9;
pub const STARTING_SHIELD_RESISTANCE: usize = 10;
pub const TIREDNESS_TO_LOSE_ENERGY: usize = 6;
pub const ATTACK_DAMAGE: usize = 3;

#[derive(Clone, Copy)]
pub enum GameCell {
    Empty,
    Bot(Bot),
    Resource(Resource),
}

pub struct Battle {
    pub state: GameState,
    pub colors: Vec<ColorConfig>,
}

#[derive(Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct GameState {
    pub map_width: usize,
    pub map_height: usize,
    pub bots: Vec<(Position, Bot)>,
    pub resources: Vec<(Position, Resource)>,
}

pub(crate) fn state_to_matrix(state: GameState) -> [[GameCell; MAP_HEIGHT]; MAP_WIDTH] {
    let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

    for (pos, bot) in state.bots {
        map[pos.x][pos.y] = GameCell::Bot(bot);
    }
    for (pos, resource) in state.resources {
        map[pos.x][pos.y] = GameCell::Resource(resource);
    }

    map
}

pub(crate) fn from_matrix(matrix: [[GameCell; MAP_HEIGHT]; MAP_WIDTH]) -> GameState {
    let mut state = GameState {
        bots: vec![],
        resources: vec![],
        map_width: MAP_WIDTH,
        map_height: MAP_HEIGHT,
    };

    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            match matrix[x][y] {
                GameCell::Bot(bot) => state.bots.push((Position { x, y }, bot)),
                GameCell::Resource(resource) => state.resources.push((Position { x, y }, resource)),
                _ => {}
            }
        }
    }

    state
}

impl Battle {
    pub fn new(colors: Vec<ColorConfig>) -> Battle {
        let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

        for color_config in colors.iter() {
            for _ in 0..color_config.number_of_bots {
                if let Some(Position { x, y }) = find_empty_position(&map) {
                    map[x][y] = GameCell::Bot(Bot::new(color_config.color));
                }
            }
        }

        let state = from_matrix(map);

        Battle { state, colors }
    }

    fn strategy_for(&self, color: Color) -> Option<BotStrategy> {
        self.colors
            .iter()
            .find(|c| c.color == color)
            .map(|c| c.strategy)
    }

    pub fn update(&mut self) {
        let old_map = state_to_matrix(self.state.clone());

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = old_map[x][y] {
                    if let Some(strategy) = self.strategy_for(bot.color) {
                        let actuators = (strategy)(&self.state, Position { x, y });
                        self.state = actuators.execute(x, y, self.state.clone());
                    }
                }
            }
        }
        let mut map = state_to_matrix(self.state.clone());

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(mut bot) = map[x][y] {
                    if bot.energy <= 0 {
                        map[x][y] = GameCell::Empty;
                    }
                }
            }
        }
        let mut rng = rand::thread_rng();
        let generated_resources = rng.gen_range(0..(AVERAGE_RESOURCE_GENERATION_PER_TICK * 2));

        if self.state.resources.len() < MAX_RESOURCES {
            for _ in 0..generated_resources {
                if let Some(Position { x, y }) = find_empty_position(&map) {
                    let energy_gain =
                        rng.gen_range(RESOURCE_MIN_ENERGY_GAIN..RESOURCE_MAX_ENERGY_GAIN);

                    map[x][y] = GameCell::Resource(Resource { energy_gain });
                }
            }
        }

        self.state = from_matrix(map);
    }
}

fn find_empty_position(map: &[[GameCell; MAP_HEIGHT]; MAP_WIDTH]) -> Option<Position> {
    let mut rng = rand::thread_rng();

    loop {
        let x: usize = rng.gen_range(0..MAP_WIDTH);
        let y: usize = rng.gen_range(0..MAP_HEIGHT);

        if let GameCell::Empty = map[x][y] {
            return Some(Position { x, y });
        }
    }
}
