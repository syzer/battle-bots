use rand::Rng;
use ruscii::terminal::Color;

use super::{
    action::ExecutableAction,
    bot::{Bot, BotStrategy, ColorConfig},
    resource::Resource,
};

pub const RESOURCE_RATE: f32 = 0.002;
pub const RESOURCE_MAX_ENERGY_GAIN: usize = 4;
pub const RESOURCE_MIN_ENERGY_GAIN: usize = 2;
pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 80;
pub const BOTS_STARTING_ENERGY: usize = 9;

#[derive(Clone, Copy)]
pub enum GameCell {
    Empty,
    Bot(Bot),
    Resource(Resource),
}

pub struct GameState {
    pub map: [[GameCell; MAP_HEIGHT]; MAP_WIDTH],
    pub colors: Vec<ColorConfig>,
}

impl GameState {
    pub fn new(colors: Vec<ColorConfig>) -> GameState {
        let mut map = [[GameCell::Empty; MAP_HEIGHT]; MAP_WIDTH];

        for color_config in colors.iter() {
            for bot in 0..color_config.number_of_bots {
                let mut rng = rand::thread_rng();

                let mut placed = false;

                while !placed {
                    let x: usize = rng.gen_range(0..MAP_WIDTH);
                    let y: usize = rng.gen_range(0..MAP_HEIGHT);

                    if let GameCell::Empty = map[x][y] {
                        map[x][y] = GameCell::Bot(Bot::new(color_config.color));
                        placed = true;
                    }
                }
            }
        }

        GameState { map, colors }
    }

    fn strategy_for(&self, color: Color) -> Option<BotStrategy> {
        self.colors
            .iter()
            .find(|c| c.color == color)
            .map(|c| c.strategy)
    }

    pub fn update(&mut self) {
        let old_map = self.map.clone();

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(bot) = old_map[x][y] {
                    if let Some(strategy) = self.strategy_for(bot.color) {
                        if let Ok(action) = (strategy)(x, y, self) {
                            action.execute(x, y, self);
                        }
                    }
                }
            }
        }
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                if let GameCell::Bot(mut bot) = self.map[x][y] {
                    if bot.energy <= 0 {
                        self.map[x][y] = GameCell::Empty;
                    }
                } else if let GameCell::Empty = self.map[x][y] {
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0.0..1.0) < RESOURCE_RATE {
                        let energy_gain =
                            rng.gen_range(RESOURCE_MIN_ENERGY_GAIN..RESOURCE_MAX_ENERGY_GAIN);

                        self.map[x][y] = GameCell::Resource(Resource { energy_gain });
                    }
                }
            }
        }
    }
}
